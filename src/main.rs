extern crate parity_wasm;
extern crate structopt;

use structopt::StructOpt;

mod app;
mod code_builder;
mod decompile;
mod engine;
mod expr_builder;
mod fingerprint;
mod fingerprint_registry;
mod format;
mod forwarder;
mod helper_semantics;
mod precedence;
mod reorder_analysis;
mod rewrites;
mod sdk;
mod semantic_resolver;
mod wasm_ir;

fn main() {
    let opt = app::Opt::from_args();
    if let Err(err) = app::run(opt) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
