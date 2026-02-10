use regex::Regex;
use soroban_sdk::Val;
use std::collections::HashMap;

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn transform_from_soroban_val_raw(val: u64) -> String {
    let mut fmt = format!("{}", val);
    let v = Val::from_payload(val);
    if v.is_good() {
        fmt = format!("{:?}", v);
        let re = Regex::new(r"\((?<value>\w+)\)").unwrap();
        if let Some(captures) = re.captures(fmt.as_str()) {
            if let Some(value) = captures.get(1) {
                fmt = format!("{}", value.as_str());
            }
        }
        replace_format_string(&mut fmt);
    }
    fmt
}

pub fn transform_from_soroban_val(val: u64) -> String {
    let raw = transform_from_soroban_val_raw(val);
    sanitize_val_literal(val, &raw)
}

fn sanitize_val_literal(val: u64, raw: &str) -> String {
    match raw {
        "True" => return "1 /* True */".to_string(),
        "False" => return "0 /* False */".to_string(),
        "Void" => return "0 /* Void */".to_string(),
        "Symbol()" => return "0 /* Symbol() */".to_string(),
        _ => {}
    }
    if raw.contains('#') {
        return format!("{} /* {} */", val, raw);
    }

    if let Some(rest) = raw.strip_prefix("Error(Contract, #") {
        if let Some(num) = rest.strip_suffix(")") {
            return format!("0 /* Error(Contract, #{}) */", num);
        }
    }

    for prefix in ["I32(", "U32(", "I64(", "U64("] {
        if let Some(rest) = raw.strip_prefix(prefix) {
            if let Some(num) = rest.strip_suffix(")") {
                return format!("{} /* {} */", num, raw);
            }
        }
    }

    raw.to_string()
}

fn replace_format_string(t: &mut String) {
    let mut replacements = HashMap::new();
    replacements.insert(
        format!("{}", INSTANCE_LIFETIME_THRESHOLD),
        "INSTANCE_LIFETIME_THRESHOLD".to_string(),
    );
    replacements.insert(
        format!("{}", BALANCE_BUMP_AMOUNT),
        "BALANCE_BUMP_AMOUNT".to_string(),
    );
    replacements.insert(
        format!("{}", BALANCE_LIFETIME_THRESHOLD),
        "BALANCE_LIFETIME_THRESHOLD".to_string(),
    );
    replacements.insert(
        format!("{}", INSTANCE_BUMP_AMOUNT),
        "INSTANCE_BUMP_AMOUNT".to_string(),
    );
    for (format_string, replacement) in &replacements {
        if *t == *format_string {
            *t = replacement.clone();
            break;
        }
    }
}
