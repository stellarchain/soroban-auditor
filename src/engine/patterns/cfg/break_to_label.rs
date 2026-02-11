use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct BreakToLabelPattern;

impl BreakToLabelPattern {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone, Debug)]
struct Frame {
    is_loop: bool,
    label: Option<String>,
}

impl Pattern for BreakToLabelPattern {
    fn name(&self) -> &'static str {
        "break_to_label"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut out = Vec::with_capacity(block.body.len());
        let mut stack: Vec<Frame> = Vec::new();

        for line in &block.body {
            let trimmed = line.trim();

            if trimmed == "} else {" {
                if !stack.is_empty() {
                    stack.pop();
                }
                stack.push(Frame {
                    is_loop: false,
                    label: None,
                });
                out.push(line.clone());
                continue;
            }

            if trimmed == "}" {
                if !stack.is_empty() {
                    stack.pop();
                }
                out.push(line.clone());
                continue;
            }

            if trimmed == "break;" && !has_active_loop(&stack) {
                if let Some(label) = nearest_label(&stack) {
                    let indent = line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>();
                    out.push(format!("{indent}break '{label};"));
                    changed = true;
                    continue;
                }
            }

            out.push(line.clone());

            if trimmed.ends_with('{') {
                stack.push(parse_frame(trimmed));
            }
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

fn has_active_loop(stack: &[Frame]) -> bool {
    stack.iter().rev().any(|f| f.is_loop)
}

fn nearest_label(stack: &[Frame]) -> Option<String> {
    stack
        .iter()
        .rev()
        .find_map(|f| f.label.as_ref().map(|s| s.to_string()))
}

fn parse_frame(trimmed: &str) -> Frame {
    let is_loop = trimmed.contains(": loop {") || trimmed == "loop {";
    let label = if let Some(rest) = trimmed.strip_prefix('\'') {
        let mut label = String::new();
        for ch in rest.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                label.push(ch);
            } else {
                break;
            }
        }
        if label.is_empty() { None } else { Some(label) }
    } else {
        None
    };
    Frame { is_loop, label }
}
