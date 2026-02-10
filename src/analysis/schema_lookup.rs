use tower_lsp::lsp_types::Location;

pub fn has_schema_key(map: &dashmap::DashMap<String, Vec<Location>>, key_upper: &str) -> bool {
    map.contains_key(key_upper)
        || map
            .iter()
            .any(|entry| entry.key().eq_ignore_ascii_case(key_upper))
}

pub fn pick_single_location(locations: &[Location]) -> Option<Location> {
    locations.iter().cloned().min_by(|a, b| {
        a.uri
            .as_str()
            .cmp(b.uri.as_str())
            .then(a.range.start.line.cmp(&b.range.start.line))
            .then(a.range.start.character.cmp(&b.range.start.character))
    })
}

pub fn lookup_schema_location(
    defs: &dashmap::DashMap<String, Vec<Location>>,
    symbol_upper: &str,
) -> Option<Location> {
    if let Some(locations) = defs.get(symbol_upper)
        && let Some(location) = pick_single_location(locations.value())
    {
        return Some(location);
    }

    defs.iter().find_map(|entry| {
        if entry.key().eq_ignore_ascii_case(symbol_upper) {
            pick_single_location(entry.value())
        } else {
            None
        }
    })
}
