pub fn format_type_ident(raw: &str) -> String {
    let mut s = raw.to_string();
    s = s.replace(" :: ", "::");
    s = s.replace(" ::", "::");
    s = s.replace(":: ", "::");
    s = s.replace("< ", "<");
    s = s.replace(" <", "<");
    s = s.replace(" >", ">");
    s = s.replace("> ", ">");
    s = s.replace(" (", "(");
    s = s.replace(" ,", ",");
    while s.contains("  ") {
        s = s.replace("  ", " ");
    }
    // Normalize tuple/type artifacts early, at type-format source, so
    // signature postprocess doesn't need to repair them later.
    while s.contains(",),") {
        s = s.replace(",),", "),");
    }
    while s.contains(",) ->") {
        s = s.replace(",) ->", ") ->");
    }
    while s.contains(",) {") {
        s = s.replace(",) {", ") {");
    }
    while s.contains(",)") {
        s = s.replace(",)", ")");
    }
    s
}

pub fn format_spec_tokens(raw: &str) -> String {
    let mut s = format_type_ident(raw);
    s = s.replace("# [", "#[");
    s = s.replace("] # [", "]\n#[");
    s = s.replace("] #[", "]\n#[");
    s = s.replace("contracttype (", "contracttype(");
    s = s.replace("contracterror (", "contracterror(");
    s = s.replace("derive (", "derive(");
    s = s.replace(" : ", ": ");
    s = s.replace(" :", ":");
    s = s.replace(")] pub", ")]\npub");
    s
}
