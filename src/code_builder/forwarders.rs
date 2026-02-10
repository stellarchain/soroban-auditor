use crate::code_builder::sdk_map::map_imported_call;
use crate::forwarder::map_forwarder_args;
use crate::wasm_ir::Function;
use std::collections::HashSet;

pub fn resolve_forwarder_chain(
    mut fn_index: u32,
    args: &[String],
    forwarders: &std::collections::BTreeMap<u32, crate::forwarder::CallForwarder>,
    import_count: usize,
    functions: &[Function],
    data_segments: &[crate::decompile::DataSegment],
) -> Option<String> {
    let mut current_args: Vec<String> = args.to_vec();
    let mut visited = HashSet::new();
    let mut depth = 0usize;

    loop {
        if (fn_index as usize) < import_count {
            let name = &functions[fn_index as usize].name;
            return Some(
                map_imported_call(name, &current_args, data_segments)
                    .unwrap_or_else(|| format!("/* TODO: host call {} */ 0", name)),
            );
        }

        if depth > 12 || !visited.insert(fn_index) {
            return None;
        }

        if let Some(forwarder) = forwarders.get(&fn_index) {
            if let Some(next_args) = map_forwarder_args(&forwarder.args, &current_args) {
                current_args = next_args;
                fn_index = forwarder.target;
                depth += 1;
                continue;
            }
            return None;
        }

        return None;
    }
}
