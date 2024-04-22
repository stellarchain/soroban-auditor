use std::error::Error;
use std::io::Read;
use lcs::LcsTable;
use levenshtein::levenshtein;
use std::{fs::File, io::BufReader};
use tlsh_fixed::{BucketKind, ChecksumKind, Tlsh, TlshBuilder, TlshError, Version};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Pattern {
    hash: String,
    pattern: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct PatternConfig {
    patterns: Vec<Pattern>,
}

pub fn search_for_patterns(function_body: &str) -> Option<String> {
    let mut replaced_body = function_body.to_string();
    match load_patterns_hash_map() {
        Ok(pattern_config) => {
            for pattern in pattern_config.patterns {
                match get_lcs_pattern(&replaced_body, &pattern.pattern) {
                    Ok(common_sequence) => {
                        match get_sequence_tlsh(&common_sequence) {
                            Ok(lcs_tlsh) => {
                                match get_sequence_tlsh(&pattern.pattern) {
                                    Ok(pattern_tlsh) => {
                                        let diff = pattern_tlsh.diff(&lcs_tlsh, false);
                                        if diff < 200 {
                                           replaced_body = replace_sequence(
                                                &replaced_body,
                                                &pattern.pattern,
                                                &pattern.body).unwrap_or(replaced_body);
                                        }
                                    }
                                    Err(err) => {
                                        println!("Error 1 loading patterns: {}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("Error 2 loading patterns: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error 3 loading patterns: {}", err);
                    }
                }
            }
            Some(replaced_body)
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

fn replace_sequence(body: &str, sequence_to_replace: &str, replacement_sequence: &str) -> Option<String> {
    let mut result = String::new();
    let mut min_distance = std::usize::MAX;
    let mut found_index = 0;
    let sequence_length = sequence_to_replace.len();
    let mut body_index = 0;

    while body_index + sequence_length <= body.len() {
        let window = &body[body_index..body_index + sequence_length];
        let distance = levenshtein(window, sequence_to_replace);
        if distance < min_distance {
            min_distance = distance;
            found_index = body_index;
        }
        body_index += 1;
    }

    if min_distance < 3 {
        result.push_str(&body[..found_index]);
        result.push_str(replacement_sequence);
        result.push_str(&body[found_index + sequence_length..]);
        Some(result)
    } else {
        None
    }
}
