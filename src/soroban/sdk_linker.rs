use lcs::LcsTable;
use levenshtein::levenshtein;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs::File, io::BufReader};
use tlsh_fixed::{BucketKind, ChecksumKind, Tlsh, TlshBuilder, TlshError, Version};

use crate::analysis::used_vars::find;

#[derive(Debug, Deserialize, Clone)]
pub struct Pattern {
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

fn calculate_indent_level(text: &str) -> usize {
    text.chars().take_while(|&c| c == ' ').count()
}

pub fn search_for_patterns(function_body: &str) -> Option<String> {
    let mut function_replaced_patterns = function_body.to_string();
    match load_patterns_hash_map() {
        Ok(mut pattern_config) => {
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
                                        println!("Error loading prefix pattern 1: {}, {}", pattern.name, err);
                                    }
                                    (_, Err(err)) => {
                                        println!("Error loading suffix pattern 2: {}, {:?}", pattern.name, err);
                                    }
                                }
                            }
                            (Err(err), _) => {
                                println!("Error loading prefix pattern 3: {}", pattern.name);
                            }
                            (_, Err(err)) => {
                                println!("Error loading suffix pattern: 4 {}", pattern.name);
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

fn calculate_pattern_score(function_body: &str, pattern: &Pattern) -> usize {
    let prefix_score = calculate_score_for_pattern(&function_body, &pattern.prefix_pattern);
    let suffix_score = calculate_score_for_pattern(&function_body, &pattern.suffix_pattern);

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

pub fn replace_sequence(pattern: &Pattern, body: &str) -> Option<String> {
    let prefix_position = find_pattern_position(body, &pattern.prefix_pattern)?;
    let suffix_position = find_pattern_position(&body[prefix_position..], &pattern.suffix_pattern)?;

    let suffix_end_position = prefix_position + suffix_position + pattern.suffix_pattern.len();

    let prefix_part = &body[..prefix_position];
    let suffix_start_position = prefix_position + suffix_position;
    let suffix_part = &body[suffix_end_position..];
    let middle_part = &body[prefix_position + pattern.prefix_pattern.len()..suffix_start_position];

    println!("{}", middle_part);
    let result = format!(
        "{}{}{}",
        prefix_part, &pattern.body_replace, suffix_part,
    );


    Some(result)
}

pub fn find_pattern_position(body: &str, pattern: &str) -> Option<usize> {
    let pattern_length = pattern.len();
    let body_length = body.len();

    // Check if the pattern length exceeds the body length
    if pattern_length > body_length {
        return None; // Pattern length exceeds body length, so no match is possible
    }

    let mut min_distance = std::usize::MAX;
    let mut found_index = None;

    // Iterate over all possible starting positions of the pattern within the body
    for i in 0..=body_length - pattern_length {
        let window = &body[i..i + pattern_length];
        let distance = levenshtein(window, pattern);

        // Update minimum distance and found index if a better match is found
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
