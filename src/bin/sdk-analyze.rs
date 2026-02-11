extern crate parity_wasm;
extern crate soroban_auditor;
extern crate structopt;

use parity_wasm::deserialize_file;
use parity_wasm::elements::{ImportCountType, Instruction};
use soroban_auditor::sdk::{get_backend, SdkFunctionDetector, SdkUsageAnalyzer};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "sdk-analyze",
    about = "Analyze Soroban SDK usage in WASM contracts"
)]
struct Opt {
    #[structopt(help = "Input WASM file", parse(from_os_str))]
    input: PathBuf,

    #[structopt(long, help = "Show detailed function-by-function breakdown")]
    detailed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // Load WASM module
    let module = deserialize_file(&opt.input)?;
    let module = module.parse_names().unwrap_or_else(|(_, m)| m);

    // Get SDK modules
    let sdk = get_backend();
    let modules = sdk.env_common_modules_result()?;

    // Build function list
    let import_count = module.import_count(ImportCountType::Function);
    let code = module.code_section().ok_or("Missing code section")?;
    let fns = module
        .function_section()
        .ok_or("Missing function section")?;
    let types = module.type_section().ok_or("Missing type section")?;

    let function_names = module
        .sections()
        .iter()
        .filter_map(|s| match s {
            parity_wasm::elements::Section::Name(name_section) => name_section.functions(),
            _ => None,
        })
        .next();

    let functions = soroban_auditor::app::utils::build_functions(
        &modules,
        module.import_section(),
        types,
        fns,
        function_names,
        false,
    );

    // Scan all function bodies for SDK calls
    let detector = SdkFunctionDetector::default();
    let mut total_sdk_calls: HashMap<String, usize> = HashMap::new();
    let mut per_function_calls: Vec<(String, HashMap<String, usize>)> = Vec::new();

    for (i, body) in code.bodies().iter().enumerate() {
        let fn_index = import_count + i;
        let fn_name = if fn_index < functions.len() {
            &functions[fn_index].name
        } else {
            continue;
        };

        let mut function_sdk_calls: HashMap<String, usize> = HashMap::new();

        for instr in body.code().elements() {
            if let Instruction::Call(idx) = instr {
                let idx = *idx as usize;
                if idx < import_count && idx < functions.len() {
                    let called_fn = &functions[idx].name;
                    if detector.is_sdk_function(called_fn) {
                        *total_sdk_calls.entry(called_fn.clone()).or_insert(0) += 1;
                        *function_sdk_calls.entry(called_fn.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        if !function_sdk_calls.is_empty() {
            per_function_calls.push((fn_name.clone(), function_sdk_calls));
        }
    }

    // Generate and print report
    let analyzer = SdkUsageAnalyzer::default();
    let report = analyzer.analyze(&total_sdk_calls);

    println!("{}", analyzer.format_report(&report));

    if opt.detailed && !per_function_calls.is_empty() {
        println!("\n=== Per-Function SDK Usage ===\n");
        for (fn_name, calls) in per_function_calls {
            println!("Function: {}", fn_name);
            let mut sorted_calls: Vec<_> = calls.iter().collect();
            sorted_calls.sort_by(|a, b| b.1.cmp(a.1));
            for (sdk_fn, count) in sorted_calls {
                println!("  {} × {}", count, sdk_fn);
            }
            println!();
        }
    }

    Ok(())
}
