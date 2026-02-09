extern crate parity_wasm;
extern crate structopt;

use structopt::StructOpt;

mod app;
mod code_builder;
mod expr_builder;
mod decompile;
mod engine;
mod fingerprint;
mod format;
mod patterns;
mod postprocess;
mod sdk;
mod precedence;
mod reorder_analysis;
mod rewrites;
mod soroban;
mod wasm_ir;

fn main() {
    let opt = app::Opt::from_args();
    if let Err(err) = app::run(opt) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
