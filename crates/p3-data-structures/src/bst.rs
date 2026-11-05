//! Binary search tree with Option<Box<Node>>.
//! Stretch: remove(), generic over T: Ord, in-order Iterator, height().
#[derive(Debug, Default)]
pub struct Bst {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Bst {
    pub fn new() -> Self { todo!() }
    /// Returns false if the key was already present.
    pub fn insert(&mut self, key: i32) -> bool { todo!() }
    pub fn contains(&self, key: i32) -> bool { todo!() }
    pub fn in_order(&self) -> Vec<i32> { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_contains_order() {
        let mut t = Bst::new();
        for k in [5, 3, 8, 1, 4, 9] { assert!(t.insert(k)); }
        assert!(!t.insert(5));
        assert!(t.contains(4));
        assert!(!t.contains(7));
        assert_eq!(t.in_order(), vec![1, 3, 4, 5, 8, 9]);
    }
}
