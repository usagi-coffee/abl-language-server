#[derive(Clone, Copy, PartialEq, Eq)]
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
