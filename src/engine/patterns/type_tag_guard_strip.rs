use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct TypeTagGuardStripPattern;

impl TypeTagGuardStripPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for TypeTagGuardStripPattern {
    fn name(&self) -> &'static str {
        "type_tag_guard_strip"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 5 {
            return None;
        }

        let mut out = Vec::with_capacity(block.body.len());
        let mut changed = false;
        let mut i = 0usize;
        while i < block.body.len() {
            if let Some((consumed, replacement)) = rewrite_at(&block.body, i) {
                out.extend(replacement);
                i += consumed;
                changed = true;
                continue;
            }
            out.push(block.body[i].clone());
            i += 1;
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body: out,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn rewrite_at(lines: &[String], start: usize) -> Option<(usize, Vec<String>)> {
    if let Some(m) = rewrite_nested_neq_cluster(lines, start) {
        return Some(m);
    }
    rewrite_sequential_eq_neq_cluster(lines, start)
}

fn rewrite_nested_neq_cluster(lines: &[String], start: usize) -> Option<(usize, Vec<String>)> {
    // Shape:
    // if x != 14 {
    //     if x != 74 {
    //         unreachable!();
    //     }
    // }
    if start + 4 >= lines.len() {
        return None;
    }

    let l0 = lines[start].trim();
    let l1 = lines[start + 1].trim();
    let l2 = lines[start + 2].trim();
    let l3 = lines[start + 3].trim();
    let l4 = lines[start + 4].trim();

    let var_a = parse_if_neq_const(l0, 14)?;
    let var_b = parse_if_neq_const(l1, 74)?;
    if var_a != var_b {
        return None;
    }
    if l2 != "unreachable!();" || l3 != "}" || l4 != "}" {
        return None;
    }

    Some((5, Vec::new()))
}

fn rewrite_sequential_eq_neq_cluster(lines: &[String], start: usize) -> Option<(usize, Vec<String>)> {
    // Shape:
    // if x == 14 {
    //     unreachable!();
    // }
    // if x != 74 {
    //     unreachable!();
    // }
    if start + 5 >= lines.len() {
        return None;
    }

    let l0 = lines[start].trim();
    let l1 = lines[start + 1].trim();
    let l2 = lines[start + 2].trim();
    let l3 = lines[start + 3].trim();
    let l4 = lines[start + 4].trim();
    let l5 = lines[start + 5].trim();

    let var_a = parse_if_eq_const(l0, 14)?;
    let var_b = parse_if_neq_const(l3, 74)?;
    if var_a != var_b {
        return None;
    }
    if l1 != "unreachable!();" || l2 != "}" {
        return None;
    }
    if l4 != "unreachable!();" || l5 != "}" {
        return None;
    }

    Some((6, Vec::new()))
}

fn parse_if_neq_const(line: &str, k: i32) -> Option<String> {
    let prefix = "if ";
    let suffix = " {";
    let inner = line.strip_prefix(prefix)?.strip_suffix(suffix)?.trim();
    let rhs = format!("!= {k}");
    let var = inner.strip_suffix(&rhs)?.trim();
    if var.is_empty() {
        return None;
    }
    Some(var.to_string())
}

fn parse_if_eq_const(line: &str, k: i32) -> Option<String> {
    let prefix = "if ";
    let suffix = " {";
    let inner = line.strip_prefix(prefix)?.strip_suffix(suffix)?.trim();
    let rhs = format!("== {k}");
    let var = inner.strip_suffix(&rhs)?.trim();
    if var.is_empty() {
        return None;
    }
    Some(var.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_simple_type_tag_guard_cluster() {
        let p = TypeTagGuardStripPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if b != 14 {".to_string(),
                "        if b != 74 {".to_string(),
                "            unreachable!();".to_string(),
                "        }".to_string(),
                "    }".to_string(),
                "    return x;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body, vec!["    return x;".to_string()]);
    }

    #[test]
    fn removes_sequential_type_tag_guard_cluster() {
        let p = TypeTagGuardStripPattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if tag == 14 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    if tag != 74 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    return y;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body, vec!["    return y;".to_string()]);
    }
}
