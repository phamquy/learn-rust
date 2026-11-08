//! Topic: lifetimes in functions and structs.

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// Splits input on whitespace, handing out slices of the original input.
pub struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    /// Next token. The returned slice must outlive the `&mut self` borrow.
    pub fn next_token(&mut self) -> Option<&'a str> {
        let s = self.input.trim_start(); // skip leading whitespace
        if s.is_empty() {
            self.input = s;
            return None;
        }
        let end = s.find(char::is_whitespace).unwrap_or(s.len());
        let (token, rest) = s.split_at(end); // token | remainder
        self.input = rest; // advance the cursor
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_works() {
        assert_eq!(longest("ab", "abc"), "abc");
    }

    #[test]
    fn tokens_outlive_the_borrow() {
        let mut t = Tokenizer::new("  a bb  ccc ");
        let a = t.next_token();
        let b = t.next_token(); // `a` must still be usable here
        assert_eq!(a, Some("a"));
        assert_eq!(b, Some("bb"));
        assert_eq!(t.next_token(), Some("ccc"));
        assert_eq!(t.next_token(), None);
    }
}
