//! Topic: Vec, HashMap, BTreeSet and the entry API.
use std::{collections::HashMap, collections::HashSet};

/// Lower-cased whitespace-separated word frequencies. Use the entry API.
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    text.split_whitespace().for_each(|word: &str| {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    });
    map
}

fn count_known<'a>(allowed: &[&'a str], text: &str) -> HashMap<&'a str, usize> {
    // Pre-fill with 0 so every allowed key is present.
    let mut map: HashMap<&str, usize> = allowed.iter().map(|&k| (k, 0)).collect();

    for word in text.split_whitespace() {
        if let Some(count) = map.get_mut(word) {
            *count += 1;
        }
        // Words outside the set are ignored.
    }
    map
}

/// Capitalize chars
pub fn up_case(s: &str) -> String {
    return s.to_uppercase();
}

/// Sorted, de-duplicated copy of the input.
pub fn dedup_sorted(v: Vec<i32>) -> Vec<i32> {
    let mut hs: HashSet<i32> = HashSet::new();

    for x in v {
        hs.insert(x);
    }

    let mut result: Vec<i32> = hs.into_iter().collect();
    result.sort();
    result
}

pub fn dedup_string<'a>(v: &Vec<&'a str>) -> Vec<&'a str> {
    let mut hs: HashSet<&str> = HashSet::new();
    for s in v {
        hs.insert(s);
    }

    let mut result: Vec<&str> = hs.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        let m = word_count("a B b a A c");
        assert_eq!(m["a"], 3);
        assert_eq!(m["b"], 2);
        assert_eq!(m["c"], 1);
    }

    #[test]
    fn dedup_string_test() {
        let example = vec!["a", "b", "c", "a"];
        assert_eq!(dedup_string(&example), vec!["a", "b", "c"]);
    }

    #[test]
    fn dedups() {
        assert_eq!(dedup_sorted(vec![3, 1, 3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn up_case_test() {
        assert_eq!(up_case("hello!"), "HELLO!")
    }

    #[test]
    fn count_known_test() {
        let allowed = ["a", "b", "c"];
        let text = "a B b a A c d";
        let m: HashMap<&str, usize> = count_known(&allowed, text);
        assert_eq!(m["a"], 2);
        assert_eq!(m["b"], 1);
        assert_eq!(m["c"], 1);
    }
}
