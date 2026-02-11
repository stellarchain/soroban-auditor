use soroban_auditor::engine::function::{split_functions, FunctionBlock};
use soroban_auditor::engine::pipeline::default_patterns;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

fn builtin_corpus() -> Vec<FunctionBlock> {
    let mut blocks = Vec::new();
    let src = r#"
pub fn sample_loop(&mut self, env: Env) -> i64 {
    let mut a: i64 = 0;
    loop {
        if a == 4 {
            break;
        }
        a = a.wrapping_add(1);
        continue;
    }
    return a;
}

pub fn sample_vec(&mut self, env: Env, to: Symbol) -> Vec<Symbol> {
    let mut vec_builder: i64 = to as i64;
    let mut slot_var1_0_i64: i64 = Hello as i64;
    let tmp = { let mut v = Vec::<Val>::new(env); v.push_back(val_from_i64(slot_var1_0_i64)); v.push_back(val_from_i64(vec_builder)); val_to_i64(v.into_val(env)) };
    let to = tmp;
    return to;
}

pub fn sample_labels(&mut self, env: Env) -> i32 {
    let mut b: i32 = 0;
    'label1: loop {
        if b == 0 {
            b = b.wrapping_add(1);
            continue 'label1;
        }
        if b != 2 {
            break 'label1;
        }
        break;
    }
    return b;
}
"#;
    blocks.extend(split_functions(src).into_iter().filter(|b| !b.body.is_empty()));
    blocks
}

fn collect_rs_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn playground_corpus() -> Vec<FunctionBlock> {
    let mut files = Vec::new();
    collect_rs_files(Path::new("playground"), &mut files);
    files.sort();

    let mut blocks = Vec::new();
    for file in files.into_iter().take(24) {
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };
        if content.len() > 180_000 {
            continue;
        }
        blocks.extend(split_functions(&content).into_iter().filter(|b| !b.body.is_empty()));
        if blocks.len() > 120 {
            break;
        }
    }
    blocks
}

#[test]
fn every_registered_pattern_runs_without_panic_on_builtin_corpus() {
    let patterns = default_patterns();
    let corpus = builtin_corpus();
    assert!(!patterns.is_empty(), "no patterns registered");
    assert!(!corpus.is_empty(), "builtin corpus is empty");

    for pattern in patterns {
        for block in &corpus {
            let result = catch_unwind(AssertUnwindSafe(|| pattern.apply(block)));
            assert!(
                result.is_ok(),
                "pattern `{}` panicked on function `{}`",
                pattern.name(),
                block.name
            );
        }
    }
}

#[test]
fn every_registered_pattern_runs_without_panic_on_playground_corpus() {
    let patterns = default_patterns();
    let corpus = playground_corpus();
    if corpus.is_empty() {
        return;
    }

    for pattern in patterns {
        for block in &corpus {
            let result = catch_unwind(AssertUnwindSafe(|| pattern.apply(block)));
            assert!(
                result.is_ok(),
                "pattern `{}` panicked on playground function `{}`",
                pattern.name(),
                block.name
            );
        }
    }
}
