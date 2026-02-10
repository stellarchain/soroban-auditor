use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use std::collections::HashMap;

pub struct RedundantTypeCheckPattern;

impl RedundantTypeCheckPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for RedundantTypeCheckPattern {
    fn name(&self) -> &'static str {
        "redundant_type_check"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let params = parse_params_from_header(&block.header);
        if params.is_empty() {
            return None;
        }

        let mut out = Vec::<String>::new();
        let mut changed = false;
        let mut i = 0usize;

        while i < block.body.len() {
            if let Some((consumed, remove_todo)) = match_single_guard(&block.body, i, &params) {
                if remove_todo
                    && !out.is_empty()
                    && out.last().unwrap().trim_start().starts_with("// TODO: Type check for ")
                {
                    out.pop();
                }
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

fn parse_params_from_header(header: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for line in header.lines() {
        let trimmed = line.trim().trim_end_matches(',');
        let Some((lhs, rhs)) = trimmed.split_once(':') else {
            continue;
        };
        let name = lhs.trim().trim_start_matches("mut ").trim();
        let ty = rhs.trim();
        if name.is_empty() || ty.is_empty() {
            continue;
        }
        out.insert(name.to_string(), ty.to_string());
    }
    out
}

fn match_single_guard(
    lines: &[String],
    at: usize,
    params: &HashMap<String, String>,
) -> Option<(usize, bool)> {
    if at + 2 >= lines.len() {
        return None;
    }
    let line = lines[at].trim();
    let line_next = lines[at + 1].trim();
    let line_next2 = lines[at + 2].trim();

    if !line.starts_with("if (") || !line.ends_with('{') {
        return None;
    }
    let body_is_abort = line_next == "unreachable!();" || line_next == "break;";
    if !body_is_abort || line_next2 != "}" {
        return None;
    }

    let (type_name, param_name) = extract_try_from_val_check(line)?;
    let param_ty = params.get(param_name)?;
    if !type_matches(type_name, param_ty) {
        return None;
    }

    // Optional TODO line immediately above.
    let remove_todo = at > 0
        && lines[at - 1]
            .trim_start()
            .starts_with("// TODO: Type check for ");

    Some((3, remove_todo))
}

fn extract_try_from_val_check(line: &str) -> Option<(&str, &str)> {
    // Matches the common form:
    // if (!(Address::try_from_val(env, &val_from_i64(admin)).is_ok())) as i32 != 0 {
    let marker = "::try_from_val(env, &val_from_i64(";
    let pos = line.find(marker)?;
    let ty_start = line[..pos]
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == ':'))
        .map(|p| p + 1)
        .unwrap_or(0);
    let ty = line[ty_start..pos].trim();
    let tail = &line[pos + marker.len()..];
    let end = tail.find(')')?;
    let param = tail[..end].trim();
    if ty.is_empty() || param.is_empty() {
        return None;
    }
    Some((ty, param))
}

fn type_matches(type_name: &str, param_ty: &str) -> bool {
    let short = type_name.rsplit("::").next().unwrap_or(type_name);
    let ty = param_ty.replace(' ', "");
    match short {
        "Address" => ty.ends_with("Address"),
        "MuxedAddress" => ty.ends_with("MuxedAddress"),
        "String" => ty.ends_with("String"),
        "Bytes" => ty.ends_with("Bytes"),
        "BytesN" => ty.contains("BytesN<"),
        "Vec" => ty.contains("Vec<"),
        "Map" => ty.contains("Map<"),
        "Symbol" => ty.ends_with("Symbol"),
        _ => false,
    }
}
