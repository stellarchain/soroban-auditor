use crate::engine::function::FunctionBlock;
use crate::engine::pattern::Pattern;
use regex::Regex;

pub struct CopyPayloadPattern;

impl CopyPayloadPattern {
    pub fn new() -> Self {
        Self
    }
}

impl Pattern for CopyPayloadPattern {
    fn name(&self) -> &'static str {
        "copy_payload_pattern"
    }

    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock> {
        if block.body.is_empty() {
            return None;
        }

        let mut changed = false;
        let mut body = block.body.clone();

        let re_load = Regex::new(
            r"^\\s*let\\s+(?P<var>\\w+)\\s*=\\s*mload64!\\((?P<base>.+?)\\.wrapping_add\\((?P<off>\\d+)\\) as usize\\) as i64;$",
        )
        .ok()?;
        let re_load_alt = Regex::new(
            r"^\\s*let\\s+(?P<var>\\w+)\\s*=\\s*mload64!\\((?P<base>.+?) as usize \\+ (?P<off>\\d+)\\) as i64;$",
        )
        .ok()?;
        let re_store = Regex::new(
            r"^\\s*mstore64!\\((?P<base>.+?)\\.wrapping_add\\((?P<off>\\d+)\\) as usize,\\s*(?P<var>\\w+) as u64\\);$",
        )
        .ok()?;
        let re_store_alt = Regex::new(
            r"^\\s*mstore64!\\((?P<base>.+?) as usize \\+ (?P<off>\\d+),\\s*(?P<var>\\w+) as u64\\);$",
        )
        .ok()?;

        let mut i = 0usize;
        while i + 5 < body.len() {
            let l1 = body[i].trim_end().to_string();
            let l2 = body[i + 1].trim_end().to_string();
            let l3 = body[i + 2].trim_end().to_string();
            let l4 = body[i + 3].trim_end().to_string();
            let l5 = body[i + 4].trim_end().to_string();
            let l6 = body[i + 5].trim_end().to_string();

            let caps1 = re_load.captures(&l1).or_else(|| re_load_alt.captures(&l1));
            let caps2 = re_store
                .captures(&l2)
                .or_else(|| re_store_alt.captures(&l2));
            let caps3 = re_load.captures(&l3).or_else(|| re_load_alt.captures(&l3));
            let caps4 = re_store
                .captures(&l4)
                .or_else(|| re_store_alt.captures(&l4));
            let caps5 = re_load.captures(&l5).or_else(|| re_load_alt.captures(&l5));
            let caps6 = re_store
                .captures(&l6)
                .or_else(|| re_store_alt.captures(&l6));

            if let (Some(c1), Some(c2), Some(c3), Some(c4), Some(c5), Some(c6)) =
                (caps1, caps2, caps3, caps4, caps5, caps6)
            {
                let var1 = c1.name("var")?.as_str();
                let var2 = c3.name("var")?.as_str();
                let var3 = c5.name("var")?.as_str();
                if c2.name("var")?.as_str() != var1
                    || c4.name("var")?.as_str() != var2
                    || c6.name("var")?.as_str() != var3
                {
                    i += 1;
                    continue;
                }

                let src1 = c1.name("base")?.as_str().trim();
                let dst1 = c2.name("base")?.as_str().trim();
                let src2 = c3.name("base")?.as_str().trim();
                let dst2 = c4.name("base")?.as_str().trim();
                let src3 = c5.name("base")?.as_str().trim();
                let dst3 = c6.name("base")?.as_str().trim();

                let off1: i32 = c1.name("off")?.as_str().parse().ok()?;
                let off2: i32 = c3.name("off")?.as_str().parse().ok()?;
                let off3: i32 = c5.name("off")?.as_str().parse().ok()?;
                let off4: i32 = c2.name("off")?.as_str().parse().ok()?;
                let off5: i32 = c4.name("off")?.as_str().parse().ok()?;
                let off6: i32 = c6.name("off")?.as_str().parse().ok()?;

                if !(off1 == off4 && off2 == off5 && off3 == off6) {
                    i += 1;
                    continue;
                }
                if !(off1 - off2 == 8 && off2 - off3 == 8) {
                    i += 1;
                    continue;
                }
                if src1 != src2 || src1 != src3 || dst1 != dst2 || dst1 != dst3 {
                    i += 1;
                    continue;
                }

                let src_base = if off3 == 8 {
                    src1.to_string()
                } else {
                    format!("{}.wrapping_add({})", src1, off3 - 8)
                };
                let dst_base = if off6 == 8 {
                    dst1.to_string()
                } else {
                    format!("{}.wrapping_add({})", dst1, off6 - 8)
                };

                let indent = body[i]
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();
                let replacement =
                    format!("{}copy_payload_3x64({}, {});", indent, src_base, dst_base);
                body.splice(i..i + 6, [replacement]);
                changed = true;
                continue;
            }
            i += 1;
        }

        if !changed {
            return None;
        }

        // Insert helper closure once after initial lets/consts.
        let helper = [
            "        let copy_payload_3x64 = |src: i32, dst: i32| {".to_string(),
            "            let v24 = mload64!(src.wrapping_add(24) as usize) as i64;".to_string(),
            "            mstore64!(dst.wrapping_add(24) as usize, v24 as u64);".to_string(),
            "            let v16 = mload64!(src.wrapping_add(16) as usize) as i64;".to_string(),
            "            mstore64!(dst.wrapping_add(16) as usize, v16 as u64);".to_string(),
            "            let v8 = mload64!(src.wrapping_add(8) as usize) as i64;".to_string(),
            "            mstore64!(dst.wrapping_add(8) as usize, v8 as u64);".to_string(),
            "        };".to_string(),
        ];
        if !body.iter().any(|l| l.contains("let copy_payload_3x64")) {
            let insert_at = body
                .iter()
                .position(|line| {
                    let t = line.trim_start();
                    !(t.starts_with("let ") || t.starts_with("const "))
                })
                .unwrap_or(body.len());
            body.splice(insert_at..insert_at, helper.clone());
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
