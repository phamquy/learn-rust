//! Topic: Vec, HashMap, BTreeSet and the entry API.
use std::collections::HashMap;

/// Lower-cased whitespace-separated word frequencies. Use the entry API.
// pub fn word_count(text: &str) -> HashMap<String, usize> {
//     todo!()
// }

/// Capitalize chars
pub fn up_case(s: &str) -> String {
    return s.to_uppercase();
}

// /// Sorted, de-duplicated copy of the input.
// pub fn dedup_sorted(v: Vec<i32>) -> Vec<i32> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn counts_words() {
    //     let m = word_count("a B b a A c");
    //     assert_eq!(m["a"], 3);
    //     assert_eq!(m["b"], 2);
    //     assert_eq!(m["c"], 1);
    // }

    // #[test]
    // fn dedups() {
    //     assert_eq!(dedup_sorted(vec![3, 1, 3, 2, 1]), vec![1, 2, 3]);
    // }

    #[test]
    fn up_case_test() {
        assert_eq!(up_case("hello!"), "HELLO!")
    }
}
