//! Topic: Result, custom error types, `?`, From, Display.
//! Do NOT use anyhow/thiserror here; build it by hand once so you know what they generate.
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingEquals,
    EmptyKey,
    BadNumber(ParseIntError),
}

// Stub so the crate compiles; replace the todo!() with a real message per variant.
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// TODO: impl std::error::Error for ParseError
// TODO: impl From<ParseIntError> for ParseError so `?` works

/// Parse "key=42" into ("key", 42).
pub fn parse_kv(s: &str) -> Result<(String, i32), ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(parse_kv("a=42"), Ok(("a".to_string(), 42)));
    }

    #[test]
    fn errors() {
        assert_eq!(parse_kv("a"), Err(ParseError::MissingEquals));
        assert_eq!(parse_kv("=1"), Err(ParseError::EmptyKey));
        assert!(matches!(parse_kv("a=x"), Err(ParseError::BadNumber(_))));
    }

    #[test]
    fn display_is_implemented() {
        let msg = ParseError::EmptyKey.to_string();
        assert!(!msg.is_empty());
    }
}
