//! Project: todo CLI with file persistence.
//! Format first as plain lines ("[ ] title" / "[x] title"); migrate to serde_json in the stretch goal.
//! Subcommands: add <title>, list, done <id>, rm <id>.

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Default)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn add(&mut self, title: &str) -> u32 {
        todo!()
    }

    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        todo!()
    }

    pub fn remove(&mut self, id: u32) -> Result<Task, String> {
        todo!()
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn serialize(&self) -> String {
        todo!()
    }

    pub fn deserialize(s: &str) -> Result<Self, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_complete_remove() {
        let mut l = TodoList::default();
        let a = l.add("a");
        let b = l.add("b");
        assert_ne!(a, b);
        l.complete(a).unwrap();
        assert!(l.tasks()[0].done);
        assert_eq!(l.remove(b).unwrap().title, "b");
        assert!(l.complete(999).is_err());
    }

    #[test]
    fn roundtrip() {
        let mut l = TodoList::default();
        l.add("write tests");
        let id = l.add("ship it");
        l.complete(id).unwrap();
        let back = TodoList::deserialize(&l.serialize()).unwrap();
        assert_eq!(back.tasks(), l.tasks());
    }
}
