use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct FunctionSignaturePattern;

impl FunctionSignaturePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for FunctionSignaturePattern {
    fn name(&self) -> &'static str {
        "function_signature"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        let header = block.header.trim_end();
        if header.contains('\n') {
            return None;
        }
        let trimmed = header.trim_start();
        if !trimmed.starts_with("pub fn ") {
            return None;
        }
        let open_pren = match header.find('(') {
            Some(idx) => idx,
            None => return None,
        };
        let close_pren = match find_matching_paren(header, open_pren) {
            Some(idx) => idx,
            None => return None,
        };
        let args_slice = &header[open_pren + 1..close_pren];
        if !args_slice.contains(',') && !args_slice.contains('&') {
            return None;
        }
        let indent = header.chars().take_while(|c| c.is_whitespace()).collect::<String>();
        let prefix = &header[..=open_pren];
        let suffix = &header[close_pren..];
        let args: Vec<String> = split_top_level_args(args_slice)
            .into_iter()
            .map(|part| part.trim().to_string())
            .filter(|part| !part.is_empty())
            .collect();
        if args.is_empty() {
            return None;
        }
        let mut new_header = String::new();
        new_header.push_str(prefix);
        new_header.push('\n');
        for (idx, arg) in args.iter().enumerate() {
            let comma = if idx + 1 < args.len() { "," } else { "" };
            new_header.push_str(&format!("{}        {}{}\n", indent, arg, comma));
        }
        let suffix = suffix.trim_start_matches(')').trim_start();
        let suffix = if suffix.starts_with("->") {
            format!(" {suffix}")
        } else {
            suffix.to_string()
        };
        new_header.push_str(&format!("{indent}){suffix}"));
        Some(FunctionBlock {
            header: new_header,
            body: block.body.clone(),
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn find_matching_paren(s: &str, open_idx: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (idx, ch) in s.char_indices().skip(open_idx) {
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(idx);
            }
        }
    }
    None
}

fn split_top_level_args(args: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut round = 0i32;
    let mut angle = 0i32;
    let mut square = 0i32;
    for (idx, ch) in args.char_indices() {
        match ch {
            '(' => round += 1,
            ')' => round -= 1,
            '<' => angle += 1,
            '>' => angle -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            ',' if round == 0 && angle == 0 && square == 0 => {
                parts.push(args[start..idx].to_string());
                start = idx + ch.len_utf8();
            }
            _ => {}
        }
    }
    if start <= args.len() {
        parts.push(args[start..].to_string());
    }
    parts
}
