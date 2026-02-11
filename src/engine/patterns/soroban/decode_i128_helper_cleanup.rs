use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct DecodeI128HelperCleanupPattern;

impl DecodeI128HelperCleanupPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for DecodeI128HelperCleanupPattern {
    fn name(&self) -> &'static str {
        "decode_i128_helper_cleanup"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.name != "decode_i128_parts" || block.body.len() < 12 {
            return None;
        }
        let lines = &block.body;

        let Some(label0_idx) = lines.iter().position(|l| l.trim().starts_with("'label0: {")) else {
            return None;
        };
        let Some(label1_idx) = lines
            .iter()
            .enumerate()
            .skip(label0_idx + 1)
            .find_map(|(i, l)| if l.trim().starts_with("'label1: {") { Some(i) } else { None }) else {
            return None;
        };

        let Some(assign_idx) = lines
            .iter()
            .enumerate()
            .skip(label1_idx + 1)
            .find_map(|(i, l)| {
                let t = l.trim();
                if t.starts_with("a = ") && t.ends_with(" & 255;") {
                    Some(i)
                } else {
                    None
                }
            }) else {
            return None;
        };

        let Some(if69_idx) = lines
            .iter()
            .enumerate()
            .skip(assign_idx + 1)
            .find_map(|(i, l)| if l.trim() == "if a != 69 {" { Some(i) } else { None }) else {
            return None;
        };
        let Some(if11_idx) = lines
            .iter()
            .enumerate()
            .skip(if69_idx + 1)
            .find_map(|(i, l)| if l.trim() == "if a != 11 {" { Some(i) } else { None }) else {
            return None;
        };
        let Some(break_label1_idx) = lines
            .iter()
            .enumerate()
            .skip(if11_idx + 1)
            .find_map(|(i, l)| if l.trim() == "break 'label1;" { Some(i) } else { None }) else {
            return None;
        };

        let Some(else69_idx) = lines
            .iter()
            .enumerate()
            .skip(break_label1_idx + 1)
            .find_map(|(i, l)| if l.trim() == "} else {" { Some(i) } else { None }) else {
            return None;
        };
        let Some(break_label0_idx) = lines
            .iter()
            .enumerate()
            .skip(else69_idx + 1)
            .find_map(|(i, l)| if l.trim() == "break 'label0;" { Some(i) } else { None }) else {
            return None;
        };

        // Find error line in the fallback arm after label1 closes.
        let mut error_idx = None;
        for (i, line) in lines.iter().enumerate().skip(break_label0_idx + 1) {
            let t = line.trim();
            if t.starts_with("let mut error_code = ") || t.starts_with("error_code = ") {
                error_idx = Some(i);
                break;
            }
            if t == "}" && i > break_label0_idx + 4 {
                break;
            }
        }
        let Some(error_idx) = error_idx else {
            return None;
        };

        let indent_if = leading_ws(&lines[if69_idx]);
        let assign_line = lines[assign_idx].clone();
        let branch11: Vec<String> = lines[(break_label1_idx + 1)..else69_idx].to_vec();
        let branch69: Vec<String> = lines[(else69_idx + 1)..break_label0_idx].to_vec();
        let error_line = lines[error_idx].clone();

        let mut out: Vec<String> = Vec::new();
        out.extend_from_slice(&lines[..label0_idx]);
        out.extend_from_slice(&lines[label0_idx + 1..label1_idx + 1]);
        out.push(assign_line);
        out.push(format!("{indent_if}if a == 69 {{"));
        out.extend(branch69);
        out.push(format!("{indent_if}}} else if a == 11 {{"));
        out.extend(branch11);
        out.push(format!("{indent_if}}} else {{"));
        out.push(error_line);
        out.push(format!("{indent_if}}}"));
        out.extend_from_slice(&lines[(error_idx + 1)..]);

        Some(FunctionBlock {
            header: block.header.clone(),
            body: out,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn leading_ws(s: &str) -> String {
    s.chars().take_while(|c| c.is_whitespace()).collect()
}
