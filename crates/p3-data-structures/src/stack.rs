//! Warm-up: generic stack over Vec<T>.
#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { todo!() }
    pub fn push(&mut self, t: T) { todo!() }
    pub fn pop(&mut self) -> Option<T> { todo!() }
    pub fn peek(&self) -> Option<&T> { todo!() }
    pub fn len(&self) -> usize { todo!() }
    pub fn is_empty(&self) -> bool { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifo() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.len(), 2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }
}
