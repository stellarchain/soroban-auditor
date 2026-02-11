use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;

pub struct WasmTypeGuardPrunePattern;

impl WasmTypeGuardPrunePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for WasmTypeGuardPrunePattern {
    fn name(&self) -> &'static str {
        "wasm_type_guard_prune"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.len() < 3 {
            return None;
        }

        let mut body = block.body.clone();
        let mut changed = false;
        let mut i = 0usize;

        while i + 2 < body.len() {
            let h = body[i].trim();
            let mid = body[i + 1].trim();
            let f = body[i + 2].trim();

            if !h.starts_with("if ") || !h.ends_with(" {") {
                i += 1;
                continue;
            }
            if mid != "unreachable!();" || f != "}" {
                i += 1;
                continue;
            }

            let cond = h.trim_start_matches("if ").trim_end_matches(" {").trim();
            if !is_wasm_type_guard(cond) {
                i += 1;
                continue;
            }

            body.drain(i..=i + 2);
            changed = true;
            continue;
        }

        if !changed {
            return None;
        }

        Some(FunctionBlock {
            header: block.header.clone(),
            body,
            footer: block.footer.clone(),
            indent: block.indent.clone(),
            name: block.name.clone(),
        })
    }
}

fn is_wasm_type_guard(cond: &str) -> bool {
    cond.contains("::try_from_val(")
        || cond.contains("& 255 != 0")
        || cond.contains("& 255 == 0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_try_from_val_unreachable_guard() {
        let p = WasmTypeGuardPrunePattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if !(Address::try_from_val(env, &val_from_i64(to)).is_ok()) {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
                "    x = 1;".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        let out = p.apply(&block).expect("must apply");
        assert_eq!(out.body, vec!["    x = 1;".to_string()]);
    }

    #[test]
    fn keeps_non_type_guards() {
        let p = WasmTypeGuardPrunePattern::new();
        let block = FunctionBlock {
            header: "pub fn f() {".to_string(),
            body: vec![
                "    if loaded_val != 0 {".to_string(),
                "        unreachable!();".to_string(),
                "    }".to_string(),
            ],
            footer: "}".to_string(),
            indent: "".to_string(),
            name: "f".to_string(),
        };
        assert!(p.apply(&block).is_none());
    }
}

