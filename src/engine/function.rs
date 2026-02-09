#[derive(Debug, Clone)]
pub struct FunctionBlock {
    pub header: String,
    pub body: Vec<String>,
    pub footer: String,
    pub indent: String,
    pub name: String,
}

pub fn split_functions(input: &str) -> Vec<FunctionBlock> {
    let mut out: Vec<FunctionBlock> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            let name_src = if trimmed.starts_with("pub fn ") {
                &trimmed["pub fn ".len()..]
            } else {
                &trimmed["fn ".len()..]
            };
            let name = name_src
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect::<String>();
            let mut depth: i32 = line.chars().filter(|&c| c == '{').count() as i32
                - line.chars().filter(|&c| c == '}').count() as i32;
            let header = line.to_string();
            i += 1;
            let body_start = i;
            while i < lines.len() && depth > 0 {
                depth += lines[i].chars().filter(|&c| c == '{').count() as i32;
                depth -= lines[i].chars().filter(|&c| c == '}').count() as i32;
                i += 1;
            }
            let body_end = i.saturating_sub(1);
            let body = lines[body_start..body_end]
                .iter()
                .map(|l| (*l).to_string())
                .collect::<Vec<String>>();
            let footer = lines.get(body_end).copied().unwrap_or("").to_string();
            out.push(FunctionBlock {
                header,
                body,
                footer,
                indent,
                name,
            });
            continue;
        }
        out.push(FunctionBlock {
            header: line.to_string(),
            body: Vec::new(),
            footer: String::new(),
            indent: String::new(),
            name: String::new(),
        });
        i += 1;
    }
    out
}

pub fn join_functions(blocks: Vec<FunctionBlock>) -> String {
    let mut out: Vec<String> = Vec::new();
    for block in blocks {
        if block.body.is_empty() && block.footer.is_empty() {
            out.push(block.header);
            continue;
        }
        out.push(block.header);
        out.extend(block.body);
        out.push(block.footer);
    }
    out.join("\n")
}
