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
    last_id: u32,
}

impl TodoList {
    pub fn add(&mut self, title: &str) -> u32 {
        let id = self.last_id;
        self.last_id += 1;
        self.tasks.push(Task {id, title: title.to_string(), done: false});
        id
    }

    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        self.tasks.iter_mut().find(|t| t.id == id)
            .map(|t| t.done = true)
            .ok_or_else(|| format!("Task with id {} not found", id))
    }

    pub fn remove(&mut self, id: u32) -> Result<Task, String> {
        self.tasks.iter().position(|t| t.id == id)
            .map(|idx| self.tasks.remove(idx))
            .ok_or_else(|| format!("Task with id {} not found", id))
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn serialize(&self) -> String {
        // serde_json::to_string(&self.tasks).unwrap()
        self.tasks.iter()
                .map(|t| format!("[{}] {}", if t.done { "x" } else { " " }, t.title))
                .collect::<Vec<_>>()
                .join("\n") 
    }

    pub fn deserialize(s: &str) -> Result<Self, String> {
        // let tasks: Vec<Task> = serde_json::from_str(s).map_err(|e| e.to_string())?;
        // let last_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        // Ok(TodoList { tasks, last_id })
        let mut tasks = Vec::new();
        let mut last_id = 0;
        for line in s.lines() {
            if let Some((done, title)) = line.split_once("] ") {
                let done = done == "[x";
                let id = last_id;
                last_id += 1;
                tasks.push(Task { id, title: title.to_string(), done });
            }
        }
        Ok(TodoList { tasks, last_id })
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
