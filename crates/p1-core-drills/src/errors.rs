//! Topic: Result, custom error types, `?`, From, Display.
//! Do NOT use anyhow/thiserror here; build it by hand once so you know what they generate.
use std::fmt::{self, write};
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
        match self {
            ParseError::MissingEquals => write!(f, "Parse error: missing '=' separator"),
            ParseError::EmptyKey => write!(f, "Parse error: key must not be empty"),
            ParseError::BadNumber(e) => write!(f, "Parse error: invalid number: {e}"),
        }
    }
}

// TODO: impl std::error::Error for ParseError
impl std::error::Error for ParseError {}
// TODO: impl From<ParseIntError> for ParseError so `?` works
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::BadNumber(e)
    }
}

/// Parse "key=42" into ("key", 42).
pub fn parse_kv(s: &str) -> Result<(String, i32), ParseError> {
    let (key, value) = s.split_once("=").ok_or(ParseError::MissingEquals)?;
    if key.is_empty() {
        return Err(ParseError::EmptyKey);
    }

    let n = value.parse::<i32>()?;
    Ok((key.to_string(), n))
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
        println!("{}", ParseError::EmptyKey);
        assert!(!msg.is_empty());
    }
}
