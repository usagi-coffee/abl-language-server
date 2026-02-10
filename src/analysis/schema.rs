pub fn normalize_lookup_key(symbol: &str, allow_dash: bool) -> String {
    symbol
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_' && (!allow_dash || c != '-'))
        .to_ascii_uppercase()
}
