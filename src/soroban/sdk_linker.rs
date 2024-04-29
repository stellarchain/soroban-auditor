use lcs::LcsTable;
use levenshtein::levenshtein;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs::File, io::BufReader};
use tlsh_fixed::{BucketKind, ChecksumKind, Tlsh, TlshBuilder, TlshError, Version};

#[derive(Debug, Deserialize)]
struct Pattern {
    name: String,
    hash: String,
    prefix_pattern: String,
    suffix_pattern: String,
    body_replace: String,
}

#[derive(Debug, Deserialize)]
struct PatternConfig {
    patterns: Vec<Pattern>,
}

enum ReplacementType {
    Prefix,
    Suffix,
}

pub fn search_for_patterns(function_body: &str) -> Option<String> {
    let mut function_replaced_patterns = function_body.to_string();
    match load_patterns_hash_map() {
        Ok(pattern_config) => {
            for pattern in pattern_config.patterns {
                match (
                    get_lcs_pattern(&function_replaced_patterns, &pattern.prefix_pattern),
                    get_lcs_pattern(&function_replaced_patterns, &pattern.suffix_pattern),
                ) {
                    (Ok(prefix_common_sequence), Ok(suffix_common_sequence)) => {
                        match (
                            get_sequence_tlsh(&prefix_common_sequence),
                            get_sequence_tlsh(&suffix_common_sequence),
                        ) {
                            (Ok(prefix_tlsh), Ok(suffix_tlsh)) => {
                                match (
                                    get_sequence_tlsh(&pattern.prefix_pattern),
                                    get_sequence_tlsh(&pattern.suffix_pattern),
                                ) {
                                    (Ok(pattern_prefix_tlsh), Ok(pattern_suffix_tlsh)) => {
                                        let prefix_diff = pattern_prefix_tlsh.diff(&prefix_tlsh, false);
                                        let suffix_diff = pattern_suffix_tlsh.diff(&suffix_tlsh, false);
                                        if prefix_diff < 30  && suffix_diff < 30 {
                                             function_replaced_patterns =
                                                replace_sequence(&pattern, &function_replaced_patterns)
                                                    .unwrap_or(function_replaced_patterns);
                                        }
                                    }
                                    (Err(err), _) => {
                                        println!("Error loading prefix pattern: {}", err);
                                    }
                                    (_, Err(err)) => {
                                        println!("Error loading suffix pattern: {}", err);
                                    }
                                }
                            }
                            (Err(err), _) => {
                                println!("Error loading prefix pattern: {}", err);
                            }
                            (_, Err(err)) => {
                                println!("Error loading suffix pattern: {}", err);
                            }
                        }
                    }
                    _ => {
                        println!("Error loading patterns 3");
                    }
                }
            }
            Some(function_replaced_patterns)
        }
        Err(err) => {
            println!("Error loading patterns: {}", err);
            None
        }
    }
}

fn load_patterns_hash_map() -> Result<PatternConfig, Box<dyn std::error::Error>> {
    let file = File::open("patterns.toml")?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let pattern_config: PatternConfig = toml::from_str(&content)?;
    Ok(pattern_config)
}

pub fn get_sequence_tlsh(code: &String) -> Result<Tlsh, TlshError> {
    if code.len() < 50 {
        return Err(TlshError::MinSizeNotReached);
    }
    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(code.as_bytes());
    builder.build()
}

pub fn get_lcs_pattern(function_body: &str, pattern: &str) -> Result<String, Box<dyn Error>> {
    let body: Vec<_> = function_body.chars().collect();
    let pat: Vec<_> = pattern.chars().collect();
    let table = LcsTable::new(&body, &pat);
    let common_seq = table.longest_common_subsequence();
    let formatted = common_seq.iter().map(|&(c1, _)| c1).collect::<String>();
    Ok(formatted)
}

fn replace_sequence(pattern: &Pattern, body: &str) -> Option<String> {
    match find_best_match(body, &pattern.prefix_pattern, &pattern.suffix_pattern) {
        (Some(prefix_found_index), Some(suffix_found_index)) => {
            let prefix = &body[..prefix_found_index];
            let suffix = &body[suffix_found_index + pattern.suffix_pattern.as_str().len()..];
            let replaced_body = format!("{}{}", prefix, pattern.body_replace);
            Some(format!("{}{}", replaced_body, suffix))
        }
        _ => None,
    }
}

fn find_best_match(body: &str, prefix_pattern: &str, suffix_pattern: &str) -> (Option<usize>, Option<usize>) {
    let mut min_distance = std::usize::MAX;
    let mut found_index_prefix = None;
    let mut found_index_suffix = None;

    for i in 0..(body.len() - (prefix_pattern.len() + suffix_pattern.len())) {
        let window = &body[i..i + prefix_pattern.len() + suffix_pattern.len()];
        let distance = levenshtein(window, prefix_pattern) + levenshtein(&window[prefix_pattern.len() ..], suffix_pattern);

        if distance < min_distance {
            min_distance = distance;
            found_index_prefix = Some(i);
            found_index_suffix = Some(i + prefix_pattern.len());
        }
    }

    (found_index_prefix, found_index_suffix)
}
