//! Topic: traits, generics, trait bounds, trait objects.

pub trait Describe {
    fn describe(&self) -> String;
}

// Stubs so the crate compiles; replace each todo!().
// i32 -> "int:5", String -> "str:hi", Vec<T> -> "[int:1, int:2]"
impl Describe for i32 {
    fn describe(&self) -> String {
        todo!()
    }
}

impl Describe for String {
    fn describe(&self) -> String {
        todo!()
    }
}

impl<T: Describe> Describe for Vec<T> {
    fn describe(&self) -> String {
        todo!()
    }
}

/// Largest element, None if empty.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    todo!()
}

/// Describe every item through a trait object.
pub fn describe_all(items: &[Box<dyn Describe>]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_works() {
        assert_eq!(largest(&[1, 5, 2]), Some(5));
        assert_eq!(largest(&[1.5, 0.2]), Some(1.5));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn describe_impls() {
        assert_eq!(5i32.describe(), "int:5");
        assert_eq!(String::from("hi").describe(), "str:hi");
        assert_eq!(vec![1i32, 2].describe(), "[int:1, int:2]");
    }

    #[test]
    fn trait_objects() {
        let items: Vec<Box<dyn Describe>> = vec![Box::new(1i32), Box::new(String::from("x"))];
        assert_eq!(describe_all(&items), vec!["int:1", "str:x"]);
    }
}
