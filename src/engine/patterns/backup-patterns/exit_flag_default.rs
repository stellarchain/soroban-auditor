use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct ExitFlagDefaultAssign;

impl ExitFlagDefaultAssign {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for ExitFlagDefaultAssign {
    fn name(&self) -> &'static str {
        "exit_flag_default_assign"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }
        let mut lines = block.body.clone();
        let mut changed = false;
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            if let Some(label) = parse_exit_label(line) {
                if let Some((assign_line, if_start, if_end)) =
                    find_exit_flag_if(&lines, i + 1, &label)
                {
                    lines[i] = assign_line.clone();
                    remove_exit_flag_sets(&mut lines, &label);
                    for idx in (if_start..=if_end).rev() {
                        lines.remove(idx);
                    }
                    changed = true;
                }
            }
            i += 1;
        }
        if !changed {
            return None;
        }
        Some(FunctionBlock {
            header: block.header.clone(),
            body: lines,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn parse_exit_label(line: &str) -> Option<String> {
    let line = line.trim();
    if !line.starts_with("let mut __exit_label") || !line.ends_with("= 0;") {
        return None;
    }
    let name = line
        .trim_start_matches("let mut ")
        .split(':')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    if name.starts_with("__exit_label") {
        Some(name)
    } else {
        None
    }
}

fn find_exit_flag_if(
    lines: &[String],
    start: usize,
    label: &str,
) -> Option<(String, usize, usize)> {
    let mut i = start;
    while i + 2 < lines.len() {
        let line = lines[i].trim();
        if line == format!("if {label} == 0 {{") {
            let assign = lines[i + 1].trim();
            if assign.starts_with("var") || assign.contains("= Error(") {
                let assign_line = lines[i + 1].clone();
                let end = find_block_end(lines, i)?;
                let mut j = end + 1;
                while j < lines.len() && lines[j].trim().is_empty() {
                    j += 1;
                }
                if j < lines.len() && lines[j].trim_start().starts_with("else ") {
                    return None;
                }
                return Some((assign_line, i, end));
            }
        }
        i += 1;
    }
    None
}

fn find_block_end(lines: &[String], start: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        depth += opens - closes;
        if depth == 0 {
            return Some(idx);
        }
    }
    None
}

fn remove_exit_flag_sets(lines: &mut [String], label: &str) {
    let needle = format!("{label} = 1;");
    for line in lines.iter_mut() {
        if line.contains(&needle) {
            *line = line
                .replace(&needle, "")
                .replace("  ;", ";")
                .replace(";  ", ";");
            *line = line.replace(";  break", "; break");
            *line = line.replace("; break", "break");
        }
    }
}
