extern crate parity_wasm;
extern crate soroban_auditor;
extern crate structopt;

use parity_wasm::deserialize_file;
use parity_wasm::elements::ImportCountType;
use soroban_auditor::sdk::get_backend;
use soroban_auditor::security::SecurityAnalyzer;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "security-analyze",
    about = "Heuristic security scanner for Soroban WASM contracts"
)]
struct Opt {
    #[structopt(help = "Input WASM file", parse(from_os_str))]
    input: PathBuf,

    #[structopt(long, help = "Show detailed evidence and remediation for each finding")]
    detailed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let module = deserialize_file(&opt.input)?;
    let module = module.parse_names().unwrap_or_else(|(_, m)| m);

    let sdk = get_backend();
    let modules = sdk.env_common_modules_result()?;

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

    let analyzer = SecurityAnalyzer::default();
    let report = analyzer.analyze_module(&module, &functions, import_count, code);

    println!("{}", analyzer.format_report(&report, opt.detailed));
    Ok(())
}
