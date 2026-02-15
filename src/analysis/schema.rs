pub fn normalize_lookup_key(symbol: &str, allow_dash: bool) -> String {
    symbol
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_' && (!allow_dash || c != '-'))
        .to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::normalize_lookup_key;

    #[test]
    fn normalizes_and_trims_symbols() {
        assert_eq!(normalize_lookup_key("  customer  ", false), "CUSTOMER");
        assert_eq!(normalize_lookup_key("::order-item::", false), "ORDER-ITEM");
    }

    #[test]
    fn preserves_dash_only_when_allowed() {
        assert_eq!(normalize_lookup_key("---foo---", false), "FOO");
        assert_eq!(normalize_lookup_key("---foo---", true), "---FOO---");
    }
}
