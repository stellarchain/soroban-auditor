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
                                        if prefix_diff < 50 && suffix_diff < 50 {
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
    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::ThreeByte, Version::Version4);
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

fn replace_sequence(pattern: &Pattern, body: &String) -> Option<String> {
    let suffix_length = pattern.suffix_pattern.len();
    let mut result = String::new();
    let mut min_distance_suffix = std::usize::MAX;
    let mut found_index_suffix = 0;

    // Iterate over the body string for suffix pattern
    for i in 0..=(body.len() - suffix_length) {
        let window = &body[i..i + suffix_length];
        let distance = levenshtein(window, &pattern.suffix_pattern);

        // Update minimum distance and found index if a better match is found
        if distance < min_distance_suffix {
            min_distance_suffix = distance;
            found_index_suffix = i;
            if distance == 0 {
                break;
            }
        }
    }

    if min_distance_suffix < 5 {
        let prefix_length = pattern.prefix_pattern.len();
        let mut min_distance_prefix = std::usize::MAX;
        let mut found_index_prefix = 0;

        // Iterate over the remaining body string for prefix pattern
        for i in found_index_suffix..=(body.len() - prefix_length) {
            let window = &body[i..i + prefix_length];
            let distance = levenshtein(window, &pattern.prefix_pattern);

            // Update minimum distance and found index if a better match is found
            if distance < min_distance_prefix {
                min_distance_prefix = distance;
                found_index_prefix = i;
                if distance == 0 {
                    break;
                }
            }
        }

        // Check if the minimum distance is within the threshold for prefix pattern
        if min_distance_prefix < 5 {
            // Construct the result string with the replacement
            result.push_str(&body[..found_index_prefix]);
            result.push_str(&pattern.body_replace);
            result.push_str(&body[found_index_suffix + suffix_length..]);
            return Some(result);
        }
    }
    None
}
