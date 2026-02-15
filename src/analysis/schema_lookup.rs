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

#[cfg(test)]
mod tests {
    use super::{has_schema_key, lookup_schema_location, pick_single_location};
    use dashmap::DashMap;
    use tower_lsp::lsp_types::{Location, Position, Range, Url};

    fn loc(uri: &str, line: u32, character: u32) -> Location {
        Location {
            uri: Url::parse(uri).expect("valid uri"),
            range: Range::new(
                Position::new(line, character),
                Position::new(line, character + 1),
            ),
        }
    }

    #[test]
    fn checks_schema_key_case_insensitively() {
        let map = DashMap::<String, Vec<Location>>::new();
        map.insert("Customer".to_string(), vec![loc("file:///tmp/customer.df", 1, 1)]);

        assert!(has_schema_key(&map, "CUSTOMER"));
        assert!(has_schema_key(&map, "customer"));
        assert!(!has_schema_key(&map, "ORDERS"));
    }

    #[test]
    fn picks_deterministic_single_location() {
        let locations = vec![
            loc("file:///tmp/z.p", 2, 0),
            loc("file:///tmp/a.p", 10, 0),
            loc("file:///tmp/a.p", 3, 2),
        ];

        let picked = pick_single_location(&locations).expect("location");
        assert_eq!(picked.uri.as_str(), "file:///tmp/a.p");
        assert_eq!(picked.range.start.line, 3);
        assert_eq!(picked.range.start.character, 2);
    }

    #[test]
    fn looks_up_schema_location_case_insensitively() {
        let defs = DashMap::<String, Vec<Location>>::new();
        defs.insert(
            "customer".to_string(),
            vec![loc("file:///tmp/customer.df", 6, 4)],
        );

        let hit = lookup_schema_location(&defs, "CUSTOMER").expect("location");
        assert_eq!(hit.uri.as_str(), "file:///tmp/customer.df");
        assert_eq!(hit.range.start.line, 6);
    }
}
