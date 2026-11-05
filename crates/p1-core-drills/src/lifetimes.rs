//! Topic: lifetimes in functions and structs.

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
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
        todo!()
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
