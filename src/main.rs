use auditor::cfg::CfgBuildError;
use auditor::wasm_wrapper::wasm;
use clap::{App, Arg};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new("auditor")
        .version(VERSION)
        .arg(
            Arg::with_name("show-graph")
                .long("show-graph")
                .help("Print the constructed CFG in dot format before structuring"),
        )
        .arg(
            Arg::with_name("file")
                .help("The wasm binary to decompile")
                .required(true),
        )
        .arg(Arg::with_name("function_name").help("The index of the function to decompile"))
        .get_matches();

    let file_path = args.value_of("file").unwrap();
    let wasm = wasm::Instance::load_file(file_path);
    let _show_graph = args.is_present("show-graph");

    if let Some(func_name) = args.value_of("function_name") {
        for (i, func) in wasm.module().functions().iter().enumerate() {
            if func.name() == func_name {
                let func_index = i as u32;
                match wasm.decompile_function(func_index) {
                    Ok(()) => (),
                    Err(CfgBuildError::NoSuchFunc) => eprintln!("No function with index {}", func_index),
                    Err(CfgBuildError::FuncIsImported) => {
                        eprintln!("Function {} is imported and can not be decompiled", func_index)
                    }
                }
            }
        }
    } else {
        for (i, func) in wasm.module().functions().iter().enumerate() {
            if !func.is_imported() {
                match wasm.decompile_function(i as u32) {
                    Ok(()) => (),
                    Err(CfgBuildError::NoSuchFunc) => eprintln!("No function with index {}", i),
                    Err(CfgBuildError::FuncIsImported) => {
                        eprintln!("Function {} is imported and can not be decompiled", i)
                    }
                }
            }
        }
    }
}
