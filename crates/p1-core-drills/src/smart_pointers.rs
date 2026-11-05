//! Topic: Box, Rc, RefCell, Drop.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// Sum of all elements. Recursion over the Box is fine.
    pub fn sum(&self) -> i32 {
        todo!()
    }
}

/// Increment the shared counter by one.
pub fn increment(counter: &Rc<RefCell<i32>>) {
    todo!()
}

/// Records its name into a shared log when dropped.
pub struct Guard {
    pub name: &'static str,
    pub log: Rc<RefCell<Vec<&'static str>>>,
}

// TODO: impl Drop for Guard that pushes `self.name` into `self.log`

#[cfg(test)]
mod tests {
    use super::*;
    use List::{Cons, Nil};

    #[test]
    fn list_sum() {
        let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        assert_eq!(l.sum(), 6);
    }

    #[test]
    fn shared_counter() {
        let c = Rc::new(RefCell::new(0));
        let c2 = Rc::clone(&c);
        increment(&c);
        increment(&c2);
        assert_eq!(*c.borrow(), 2);
        assert_eq!(Rc::strong_count(&c), 2);
    }

    #[test]
    fn drop_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        {
            let _a = Guard { name: "a", log: Rc::clone(&log) };
            let _b = Guard { name: "b", log: Rc::clone(&log) };
        }
        assert_eq!(*log.borrow(), vec!["b", "a"]);
    }
}
