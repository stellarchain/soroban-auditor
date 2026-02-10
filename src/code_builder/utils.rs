use parity_wasm::elements::Instruction;
use regex::Regex;

pub fn strip_parens(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') && s.len() > 2 {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

pub fn strip_tag_expr(s: &str) -> Option<String> {
    let s = strip_parens(s).trim();
    let idx = s.rfind('&')?;
    let (left, right) = s.split_at(idx);
    if right[1..].trim() == "255" {
        Some(left.trim().to_string())
    } else {
        None
    }
}

pub fn extract_obj_type(s: &str) -> Option<String> {
    let s = s.trim();
    let start = s.find("/*")?;
    let end = s[start + 2..].find("*/")?;
    let inside = s[start + 2..start + 2 + end].trim();
    let paren = inside.find("(obj#0)")?;
    Some(inside[..paren].trim().to_string())
}

pub fn extract_obj_type_direct(s: &str) -> Option<String> {
    let s = s.trim();
    let pos = s.find("(obj#0)")?;
    let head = s[..pos].trim();
    if head.is_empty() {
        None
    } else {
        Some(head.trim_matches(|c| c == '(' || c == ')').to_string())
    }
}

pub fn try_format_tag_compare(a: &str, b: &str, equal: bool) -> Option<String> {
    let a_type = extract_obj_type(a).or_else(|| extract_obj_type_direct(a));
    let b_type = extract_obj_type(b).or_else(|| extract_obj_type_direct(b));
    let a_base = strip_tag_expr(a);
    let b_base = strip_tag_expr(b);
    let (base, ty) = if let (Some(base), Some(ty)) = (a_base, b_type) {
        (base, ty)
    } else if let (Some(base), Some(ty)) = (b_base, a_type) {
        (base, ty)
    } else {
        return None;
    };
    let check = match ty.as_str() {
        "Address" => format!(
            "Address::try_from_val(env, &val_from_i64({})).is_ok()",
            base
        ),
        "Vec" => format!(
            "Vec::<Val>::try_from_val(env, &val_from_i64({})).is_ok()",
            base
        ),
        "Map" => format!(
            "Map::<Val, Val>::try_from_val(env, &val_from_i64({})).is_ok()",
            base
        ),
        "Bytes" => format!("Bytes::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "String" => format!("String::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "Symbol" => format!("Symbol::try_from_val(env, &val_from_i64({})).is_ok()", base),
        "MuxedAddress" => {
            format!(
                "MuxedAddress::try_from_val(env, &val_from_i64({})).is_ok()",
                base
            )
        }
        _ => return None,
    };
    if equal {
        Some(check)
    } else {
        Some(format!("!({})", check))
    }
}

pub fn parse_stack_addr(addr: &str) -> Option<(String, i32)> {
    let s = addr.trim();
    let s = s.trim_start_matches('(').trim_end_matches(')');
    let s = s
        .replace(" as i32", "")
        .replace(" as i64", "")
        .replace(" as usize", "");
    if let Some(caps) = Regex::new(r"^(var\d+)$").ok()?.captures(&s) {
        return Some((caps[1].to_string(), 0));
    }
    if let Some(caps) = Regex::new(r"^(var\d+)\.wrapping_add\(([-\d]+)\)$")
        .ok()?
        .captures(&s)
    {
        let base = caps[1].to_string();
        let off = caps[2].parse::<i32>().ok()?;
        return Some((base, off));
    }
    if let Some(caps) = Regex::new(r"^(var\d+)\.wrapping_sub\(([-\d]+)\)$")
        .ok()?
        .captures(&s)
    {
        let base = caps[1].to_string();
        let off = caps[2].parse::<i32>().ok()?;
        return Some((base, -off));
    }
    None
}

pub fn slot_name(base: &str, offset: i32, suffix: &str) -> String {
    let mut name = format!("slot_{}_{}_{}", base, offset, suffix);
    name = name.replace('-', "m");
    name
}

pub fn is_breakable_if(remaining_ops: &[Instruction]) -> bool {
    let mut stack = 0;

    for opcode in remaining_ops {
        use parity_wasm::elements::Instruction::*;
        match *opcode {
            Block(_) | If(_) | Loop(_) => {
                stack += 1;
            }
            End => {
                if stack == 0 {
                    return false;
                }
                stack -= 1;
            }
            Br(relative_depth) | BrIf(relative_depth) => {
                if relative_depth == stack {
                    return true;
                }
            }
            BrTable(ref table) => {
                if table.table.iter().any(|&i| i == stack) || table.default == stack {
                    return true;
                }
            }
            _ => {}
        }
    }
    panic!("Unclosed block")
}

pub fn compute_label_needed(code: &[Instruction]) -> std::collections::HashSet<usize> {
    use parity_wasm::elements::Instruction::*;
    let mut stack: Vec<usize> = Vec::new();
    let mut needed: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for (idx, instr) in code.iter().enumerate() {
        match instr {
            Block(_) | Loop(_) | If(_) => stack.push(idx),
            Else => {}
            End => {
                stack.pop();
            }
            Br(depth) | BrIf(depth) => {
                let depth = *depth as usize;
                if depth < stack.len() {
                    let target = stack[stack.len() - 1 - depth];
                    needed.insert(target);
                }
            }
            BrTable(table) => {
                for depth in table.table.iter().copied() {
                    let depth = depth as usize;
                    if depth < stack.len() {
                        let target = stack[stack.len() - 1 - depth];
                        needed.insert(target);
                    }
                }
                let depth = table.default as usize;
                if depth < stack.len() {
                    let target = stack[stack.len() - 1 - depth];
                    needed.insert(target);
                }
            }
            _ => {}
        }
    }
    needed
}
