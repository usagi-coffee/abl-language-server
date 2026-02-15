#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BasicType {
    Character,
    Numeric,
    Logical,
    DateLike,
    Handle,
}

impl BasicType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Character => "CHARACTER",
            Self::Numeric => "NUMERIC",
            Self::Logical => "LOGICAL",
            Self::DateLike => "DATE",
            Self::Handle => "HANDLE",
        }
    }
}

pub fn builtin_type_from_name(raw: &str) -> Option<BasicType> {
    let upper = raw
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();

    match upper.as_str() {
        "CHARACTER" | "CHAR" | "LONGCHAR" | "CLOB" => Some(BasicType::Character),
        "INTEGER" | "INT" | "INT64" | "DECIMAL" | "DEC" | "NUMERIC" | "NUM" => {
            Some(BasicType::Numeric)
        }
        "LOGICAL" | "LOG" | "BOOLEAN" => Some(BasicType::Logical),
        "DATE" | "DATETIME" | "DATETIME-TZ" => Some(BasicType::DateLike),
        "HANDLE" | "COM-HANDLE" | "WIDGET-HANDLE" => Some(BasicType::Handle),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{BasicType, builtin_type_from_name};

    #[test]
    fn maps_builtin_type_aliases() {
        assert_eq!(builtin_type_from_name("char"), Some(BasicType::Character));
        assert_eq!(builtin_type_from_name("int64"), Some(BasicType::Numeric));
        assert_eq!(builtin_type_from_name("boolean"), Some(BasicType::Logical));
        assert_eq!(
            builtin_type_from_name("datetime-tz"),
            Some(BasicType::DateLike)
        );
        assert_eq!(
            builtin_type_from_name("widget-handle"),
            Some(BasicType::Handle)
        );
    }

    #[test]
    fn ignores_trailing_tokens_and_unknown_types() {
        assert_eq!(
            builtin_type_from_name("character extent"),
            Some(BasicType::Character)
        );
        assert_eq!(builtin_type_from_name("raw"), None);
    }

    #[test]
    fn returns_canonical_labels() {
        assert_eq!(BasicType::Character.label(), "CHARACTER");
        assert_eq!(BasicType::DateLike.label(), "DATE");
    }
}
