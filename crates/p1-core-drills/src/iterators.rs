//! Topic: iterators, closures, adaptors, implementing Iterator.

/// Sum of squares of the even numbers. One iterator chain, no loops.
pub fn sum_of_even_squares(v: &[i32]) -> i32 {
    todo!()
}

/// Lengths of the words, skipping empty ones.
pub fn word_lengths(words: &[&str]) -> Vec<usize> {
    todo!()
}

/// Returns a closure that adds `n` to its argument.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

/// Fibonacci sequence 0, 1, 1, 2, 3, 5, ...
pub struct Fib {
    a: u64,
    b: u64,
}

impl Fib {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_squares() {
        assert_eq!(sum_of_even_squares(&[1, 2, 3, 4]), 20);
    }

    #[test]
    fn lengths() {
        assert_eq!(word_lengths(&["a", "", "abc"]), vec![1, 3]);
    }

    #[test]
    fn adder() {
        assert_eq!(make_adder(2)(3), 5);
    }

    #[test]
    fn fib() {
        let v: Vec<u64> = Fib::new().take(7).collect();
        assert_eq!(v, vec![0, 1, 1, 2, 3, 5, 8]);
    }
}
