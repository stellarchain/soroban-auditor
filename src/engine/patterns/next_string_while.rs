use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct NextStringWhile;

impl NextStringWhile {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for NextStringWhile {
    fn name(&self) -> &'static str {
        "next_string_while"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let body = &block.body;
        let mut i = 0usize;
        while i < body.len() {
            let line = body[i].trim();
            if !line.starts_with("let (") || !line.contains("self.next_string_checked") {
                i += 1;
                continue;
            }

            let (vars, call) = match parse_next_string_line(body[i].as_str()) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            if vars.len() != 3 {
                i += 1;
                continue;
            }
            let ok_var = vars[2].clone();

            let if_idx = match next_non_empty(body, i + 1) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let if_line = body[if_idx].trim();
            if !if_line.starts_with("if (") || !if_line.contains(&format!("{ok_var} == 0")) {
                i += 1;
                continue;
            }
            let break_idx = match next_non_empty(body, if_idx + 1) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let break_line = body[break_idx].trim();
            let _label = match parse_break_label(break_line) {
                Some(l) => l,
                None => {
                    i += 1;
                    continue;
                }
            };
            let if_end_idx = match next_non_empty(body, break_idx + 1) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            if body[if_end_idx].trim() != "}" {
                i += 1;
                continue;
            }

            let loop_start = match find_enclosing_loop_start(body, i) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let loop_end = match find_matching_end(body, loop_start) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };

            let after_loop = match next_non_empty(body, loop_end + 1) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let err_line = body[after_loop].trim();
            if !err_line.contains("Error(") || !err_line.contains("=") {
                i += 1;
                continue;
            }

            let loop_indent = indentation_of(&body[loop_start]);
            let decl_indent = loop_indent.clone();
            let tmp_a = format!("{}_tmp", vars[0]);
            let tmp_b = format!("{}_tmp", vars[1]);
            let tmp_ok = format!("{}_tmp", vars[2]);
            let loop_label = parse_loop_label(&body[loop_start]);

            let setup_start = loop_start + 1;
            let setup_end = i.saturating_sub(1);
            let mut setup_lines: Vec<String> = Vec::new();
            let mut setup_ok = true;
            if setup_end >= setup_start {
                for idx in setup_start..=setup_end {
                    let line = body[idx].trim();
                    if line.is_empty() {
                        continue;
                    }
                    if line.contains('{')
                        || line.contains('}')
                        || line.starts_with("if ")
                        || line.starts_with("match ")
                        || line.starts_with("loop ")
                        || line.starts_with("while ")
                    {
                        setup_ok = false;
                        break;
                    }
                    setup_lines.push(body[idx].clone());
                }
            }
            if !setup_ok {
                i += 1;
                continue;
            }

            let mut new_body: Vec<String> = Vec::new();
            for (idx, line) in body.iter().enumerate() {
                if idx == loop_start {
                    new_body.push(format!("{decl_indent}let mut {}: i64 = 0;", vars[0]));
                    new_body.push(format!("{decl_indent}let mut {}: i64 = 0;", vars[1]));
                    new_body.push(format!("{decl_indent}let mut {}: i32 = 0;", vars[2]));
                    new_body.push(format!("{decl_indent}let mut loop_failed: i32 = 0;"));
                    let while_header = if let Some(label) = loop_label.as_deref() {
                        format!("{loop_indent}'{}: while {{", label)
                    } else {
                        format!("{loop_indent}while {{")
                    };
                    new_body.push(while_header);
                    let cond_indent = format!("{loop_indent}    ");
                    for setup_line in &setup_lines {
                        new_body.push(format!("{cond_indent}{}", setup_line.trim()));
                    }
                    new_body.push(format!(
                        "{cond_indent}let ({tmp_a}, {tmp_b}, {tmp_ok}) = {call};"
                    ));
                    new_body.push(format!("{cond_indent}{} = {tmp_a};", vars[0]));
                    new_body.push(format!("{cond_indent}{} = {tmp_b};", vars[1]));
                    new_body.push(format!("{cond_indent}{} = {tmp_ok};", vars[2]));
                    new_body.push(format!("{cond_indent}{} != 0", vars[2]));
                    new_body.push(format!("{loop_indent}}} {{"));

                    let mut depth = 0i32;
                    for inner_idx in (loop_start + 1)..loop_end {
                        if inner_idx >= setup_start && inner_idx <= i {
                            continue;
                        }
                        if inner_idx >= if_idx && inner_idx <= if_end_idx {
                            continue;
                        }
                        let inner_line = &body[inner_idx];
                        let trimmed = inner_line.trim();
                        if trimmed.ends_with('{') {
                            depth += 1;
                        }
                        if trimmed == "}" {
                            depth -= 1;
                        }
                        if depth == 0 && trimmed == "break;" {
                            let indent = indentation_of(inner_line);
                            new_body.push(format!("{indent}loop_failed = 1;"));
                            new_body.push(format!("{indent}break;"));
                            continue;
                        }
                        new_body.push(inner_line.clone());
                    }
                    new_body.push(body[loop_end].clone());
                    continue;
                }
                if idx > loop_start && idx <= loop_end {
                    continue;
                }
                if idx == after_loop {
                    let indent = indentation_of(line);
                    let err_stmt = line.trim().to_string();
                    new_body.push(format!("{indent}if loop_failed != 0 {{"));
                    new_body.push(format!("{indent}    {err_stmt}"));
                    new_body.push(format!("{indent}}}"));
                    continue;
                }
                new_body.push(line.clone());
            }

            return Some(FunctionBlock {
                header: block.header.clone(),
                body: new_body,
                footer: block.footer.clone(),
                indent: block.indent.clone(),
                name: block.name.clone(),
            });
        }

        None
    }
}

fn parse_next_string_line(line: &str) -> Option<(Vec<String>, String)> {
    let trimmed = line.trim();
    let prefix = "let (";
    if !trimmed.starts_with(prefix) {
        return None;
    }
    let close = trimmed.find(") =")?;
    let vars_str = trimmed[prefix.len()..close].trim();
    let vars = vars_str
        .split(',')
        .map(|v| v.trim().to_string())
        .collect::<Vec<_>>();
    let call_start = trimmed.find("= ")? + 2;
    let call = trimmed[call_start..].trim_end_matches(';').to_string();
    Some((vars, call))
}

fn parse_break_label(line: &str) -> Option<String> {
    let t = line.trim();
    if !t.starts_with("break 'label") || !t.ends_with(';') {
        return None;
    }
    let inner = t.trim_start_matches("break 'label").trim_end_matches(';');
    Some(inner.to_string())
}

fn next_non_empty(lines: &[String], mut idx: usize) -> Option<usize> {
    while idx < lines.len() {
        if !lines[idx].trim().is_empty() {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

fn indentation_of(line: &str) -> String {
    line.chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>()
}

fn parse_loop_label(line: &str) -> Option<String> {
    let t = line.trim();
    if let Some(pos) = t.find(": loop {") {
        let label = t[..pos].trim().trim_start_matches('\'').to_string();
        if !label.is_empty() {
            return Some(label);
        }
    }
    None
}

fn find_enclosing_loop_start(lines: &[String], idx: usize) -> Option<usize> {
    for s in (0..idx).rev() {
        let t = lines[s].trim();
        if t == "loop {" || t.ends_with(": loop {") {
            if let Some(end) = find_matching_end(lines, s) {
                if end > idx {
                    return Some(s);
                }
            }
        }
    }
    None
}

fn find_parent_label_block(lines: &[String], child_start: usize, label: &str) -> Option<usize> {
    let target = format!("'label{}: {{", label);
    for s in (0..child_start).rev() {
        if lines[s].trim() == target {
            if let Some(end) = find_matching_end(lines, s) {
                if end > child_start {
                    return Some(s);
                }
            }
        }
    }
    None
}

fn find_matching_end(lines: &[String], start: usize) -> Option<usize> {
    let mut depth: i32 = 0;
    for i in start..lines.len() {
        let mut opens = 0;
        let mut closes = 0;
        for ch in lines[i].chars() {
            if ch == '{' {
                opens += 1;
            } else if ch == '}' {
                closes += 1;
            }
        }
        depth += opens as i32;
        depth -= closes as i32;
        if i == start && opens == 0 {
            return None;
        }
        if depth == 0 && i > start {
            return Some(i);
        }
    }
    None
}
