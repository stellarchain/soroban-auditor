extern crate parity_wasm;
extern crate structopt;

use structopt::StructOpt;

// instead of redeclaring all of the modules that already live in the
// library crate, simply import them from `soroban_auditor`.  this keeps the
// binary crate small and ensures we don't duplicate code (and avoids the
// "unresolved module" errors when the library exposes helpers such as
// `helper_fingerprints`).
use soroban_auditor::{
    app,
    code_builder,
    decompile,
    engine,
    expr_builder,
    fingerprint,
    fingerprint_registry,
    format,
    forwarder,
    helper_semantics,
    precedence,
    reorder_analysis,
    rewrites,
    sdk,
    semantic_resolver,
    wasm_ir,
};

fn main() {
    let opt = app::Opt::from_args();
    if let Err(err) = app::run(opt) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
