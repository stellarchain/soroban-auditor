use lcs::LcsTable;
use levenshtein::levenshtein;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs::File, io::BufReader};
use tlsh_fixed::{BucketKind, ChecksumKind, Tlsh, TlshBuilder, TlshError, Version};

#[derive(Debug, Deserialize, Eq, Clone, Hash, PartialEq)]
pub struct Pattern {
    name: String,
    pattern: String,
    body_replace: String,
}

#[derive(Debug, Deserialize)]
struct PatternConfig {
    patterns: Vec<Pattern>,
}

pub fn search_for_patterns(function_body: &str) -> Option<Vec<(String, String)>> {
    let mut found_patterns = Vec::new();
    let mut function_replaced_patterns = function_body.to_string();
    let cleaned_function_body = function_body.replace(" ", "").replace("\n", "");
    match load_patterns_hash_map() {
        Ok(pattern_config) => {
            for pattern in &pattern_config.patterns {
                let cleaned_pattern = &pattern.pattern.replace(" ", "").replace("\n", "");
                if let Ok(common_sequence) = get_lcs_pattern(&cleaned_function_body, &cleaned_pattern) {
                    match get_sequence_tlsh(&common_sequence) {
                        Ok(tlsh_sequence) => match get_sequence_tlsh(&cleaned_pattern) {
                            Ok(pattern_tlsh) => {
                                let diff = pattern_tlsh.diff(&tlsh_sequence, false);
                                if diff < 50 {
                                    if let Some(replaced) = replace_sequence(&pattern, &function_replaced_patterns) {
                                        function_replaced_patterns = replaced;
                                        found_patterns.push((pattern.name.clone(), function_replaced_patterns.clone()));
                                        return Some(found_patterns);
                                    }
                                }
                            }
                            Err(_) => {},
                        },
                        Err(_) => {},
                    }
                } else {
                    println!("Error loading pattern for {}", pattern.name);
                }
            }
            if found_patterns.len() > 0 {
                return Some(found_patterns);
            }
            None
        }
        Err(err) => {
            println!("Error loading patterns: {}", err);
            None
        }
    }
}

fn calculate_pattern_score(function_body: &str, pattern: &Pattern) -> usize {
    let prefix_score = calculate_score_for_pattern(&function_body, &pattern.pattern);
    let suffix_score = calculate_score_for_pattern(&function_body, &pattern.pattern);

    match prefix_score.checked_add(suffix_score) {
        Some(score) => score,
        None => {
            println!("Overflow occurred while calculating pattern score.");
            std::usize::MAX // Return a large value to indicate overflow
        }
    }
}

fn calculate_score_for_pattern(function_body: &str, pattern: &str) -> usize {
    let pattern_length = pattern.len();
    if pattern_length > function_body.len() {
        return std::usize::MAX;
    }
    let mut min_distance = std::usize::MAX;
    for i in 0..=function_body.len() - pattern_length {
        let window = &function_body[i..i + pattern_length];
        let distance = levenshtein(window, pattern);

        if distance < min_distance {
            min_distance = distance;
            if distance == 0 {
                break;
            }
        }
    }
    min_distance
}

fn load_patterns_hash_map() -> Result<PatternConfig, Box<dyn std::error::Error>> {
    let file = File::open("src/soroban/patterns/sac_contract.toml")?;
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

pub fn replace_sequence(pattern: &Pattern, body: &str) -> Option<String> {
    let prefix_position = find_pattern_position(body, &pattern.pattern)?;
    let replaced = format!("{}{}", &body[..prefix_position], &pattern.body_replace);
    Some(replaced)
}

pub fn find_pattern_position(body: &str, pattern: &str) -> Option<usize> {
    let pattern_length = pattern.len();
    let body_length = body.len();

    let mut min_distance = std::usize::MAX;
    let mut found_index = None;

    for i in 0..=body_length {
        let window = if i + pattern_length <= body_length {
            &body[i..i + pattern_length]
        } else {
            &body[i..] 
        };

        let distance = levenshtein(window, pattern);

        if distance < min_distance {
            min_distance = distance;
            found_index = Some(i);
            if distance == 0 {
                break; // Exact match found, no need to search further
            }
        }
    }

    found_index
}
