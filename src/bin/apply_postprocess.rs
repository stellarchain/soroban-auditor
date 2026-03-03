use std::{env, fs, path::Path, process};

// small tool to apply the postprocess pipeline to existing .rs outputs
// also supports generating a fingerprint DB from a directory of source code
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: apply_postprocess [--gen-db <src-dir> <out.json>] <file1.rs> [file2.rs ...]");
        process::exit(1);
    }
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--gen-db" => {
                if i + 2 >= args.len() {
                    eprintln!("--gen-db requires two arguments");
                    process::exit(1);
                }
                let src = Path::new(&args[i + 1]);
                let out = Path::new(&args[i + 2]);
                match soroban_auditor::helper_fingerprints::build_fingerprint_db(src) {
                    Ok(db) => {
                        if let Err(e) = soroban_auditor::helper_fingerprints::save_db(&db, out) {
                            eprintln!("failed to write DB: {}", e);
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to scan directory {}: {}", src.display(), e);
                        process::exit(1);
                    }
                }
                i += 3;
            }
            "--extend-db" => {
                if i + 2 >= args.len() {
                    eprintln!("--extend-db requires two arguments");
                    process::exit(1);
                }
                let dbpath = Path::new(&args[i + 1]);
                let src = Path::new(&args[i + 2]);
                // load existing
                match soroban_auditor::helper_fingerprints::load_db(dbpath) {
                    Ok(mut db) => {
                        if let Err(e) = soroban_auditor::helper_fingerprints::extend_db_from_decompiled(src, &mut db) {
                            eprintln!("failed to extend DB from {}: {}", src.display(), e);
                            process::exit(1);
                        }
                        if let Err(e) = soroban_auditor::helper_fingerprints::save_db(&db, dbpath) {
                            eprintln!("failed to write extended DB: {}", e);
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to load DB {}: {}", dbpath.display(), e);
                        process::exit(1);
                    }
                }
                i += 3;
            }
            file => {
                let path = Path::new(file);
                match fs::read_to_string(path) {
                    Ok(src) => {
                        let name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("contract");
                        let processed = soroban_auditor::app::postprocess_str(src, name);
                        if let Err(e) = fs::write(path, processed) {
                            eprintln!("failed to write {}: {}", file, e);
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to read {}: {}", file, e);
                    }
                }
                i += 1;
            }
        }
    }
}
