//! Singly linked stack with Option<Box<Node<T>>>. See "Learning Rust With Entirely Too Many Linked Lists".
//! Also implement: iter() (borrowing), into_iter() (owning), and Drop (iterative, to avoid stack overflow).
pub struct LinkedStack<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self { todo!() }
    pub fn push(&mut self, t: T) { todo!() }
    pub fn pop(&mut self) -> Option<T> { todo!() }
    pub fn peek(&self) -> Option<&T> { todo!() }
    pub fn peek_mut(&mut self) -> Option<&mut T> { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut s = LinkedStack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        if let Some(v) = s.peek_mut() { *v = 20; }
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn long_list_drop_does_not_overflow() {
        let mut s = LinkedStack::new();
        for i in 0..200_000 { s.push(i); }
        drop(s);
    }
}
