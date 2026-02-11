use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct InlineVecBuilderMacroPattern;

impl InlineVecBuilderMacroPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for InlineVecBuilderMacroPattern {
    fn name(&self) -> &'static str {
        "inline_vec_builder_macro"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        for line in &block.body {
            let rewritten = rewrite_line(line);
            if rewritten != *line {
                changed = true;
            }
            out.push(rewritten);
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

fn rewrite_line(line: &str) -> String {
    let marker = "{ let mut v = Vec::<Val>::new(env);";
    let Some(start) = line.find(marker) else {
        return line.to_string();
    };
    let Some(end) = line[start..].find("val_to_i64(v.into_val(env)) }") else {
        return line.to_string();
    };
    let end = start + end + "val_to_i64(v.into_val(env)) }".len();
    let body = &line[start..end];
    let args = extract_push_args(body);
    if args.is_empty() {
        return line.to_string();
    }

    let items = args
        .iter()
        .map(|a| format!("val_from_i64({a})"))
        .collect::<Vec<_>>()
        .join(", ");
    let replacement = format!("val_to_i64(vec![&env, {items}].into_val(env))");

    let mut out = String::new();
    out.push_str(&line[..start]);
    out.push_str(&replacement);
    out.push_str(&line[end..]);
    out
}

fn extract_push_args(segment: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = segment;
    let marker = "v.push_back(val_from_i64(";
    while let Some(pos) = rest.find(marker) {
        let tail = &rest[pos + marker.len()..];
        let Some(end) = tail.find("))") else {
            break;
        };
        out.push(tail[..end].trim().to_string());
        rest = &tail[end + 2..];
    }
    out
}

