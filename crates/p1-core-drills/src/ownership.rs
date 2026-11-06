//! Topic: ownership and borrowing.

/// Return whichever string is longer. Ties return `a`. Takes ownership of both.
pub fn longest_owned(a: String, b: String) -> String {
    if a.len() >= b.len() {
        return a;
    } else {
        return b;
    }
}

pub fn test_longest_owned() {
    let a = String::from("string a");
    let b = String::from("string b");

    let c = longest_owned(a, b);
}

/// Return the first whitespace-delimited word, or "" if none. Must not allocate.
pub fn first_word(s: &str) -> &str {
    if let Some(w) = s.split_whitespace().next() {
        return w;
    }
    return &"";
}

/// Push `x` onto `v` and return it. Takes ownership, no clone.
pub fn push_and_return(v: Vec<i32>, x: i32) -> Vec<i32> {
    let mut v = v;
    v.push(x);
    v
}

/// Double every element in place through a mutable borrow.
pub fn double_in_place(v: &mut [i32]) {
    v.iter_mut().for_each(|x| *x *= 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_owned_works() {
        assert_eq!(longest_owned("ab".into(), "abc".into()), "abc");
        assert_eq!(longest_owned("ab".into(), "cd".into()), "ab");
    }

    #[test]
    fn first_word_works() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("   hi"), "hi");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn push_and_return_works() {
        assert_eq!(push_and_return(vec![1, 2], 3), vec![1, 2, 3]);
    }

    #[test]
    fn double_in_place_works() {
        let mut v = [1, 2, 3];
        double_in_place(&mut v);
        assert_eq!(v, [2, 4, 6]);
    }
}
