//! Project: todo CLI with file persistence.
//! Persist tasks as a JSON array containing their IDs, titles, and completion state.
//! Subcommands: add <title>, list, done <id>, rm <id>.

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
        self.tasks.push(Task {
            id,
            title: title.to_string(),
            done: false,
        });
        id
    }

    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .map(|t| t.done = true)
            .ok_or_else(|| format!("Task with id {} not found", id))
    }

    pub fn remove(&mut self, id: u32) -> Result<Task, String> {
        self.tasks
            .iter()
            .position(|t| t.id == id)
            .map(|idx| self.tasks.remove(idx))
            .ok_or_else(|| format!("Task with id {} not found", id))
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn serialize(&self) -> Result<String, String> {
        serde_json::to_string(&self.tasks).map_err(|e| e.to_string())
    }

    pub fn deserialize(s: &str) -> Result<Self, String> {
        let tasks: Vec<Task> = serde_json::from_str(s).map_err(|e| e.to_string())?;
        let last_id = match tasks.iter().map(|t| t.id).max() {
            Some(id) => id.checked_add(1).ok_or("Task ID limit reached")?,
            None => 0,
        };
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
        let back = TodoList::deserialize(&l.serialize().unwrap()).unwrap();
        assert_eq!(back.tasks(), l.tasks());
    }

    #[test]
    fn serialize_json() {
        let mut l = TodoList::default();
        l.add("quotes \" and newline\n");
        l.complete(0).unwrap();
        let json: serde_json::Value = serde_json::from_str(&l.serialize().unwrap()).unwrap();
        assert_eq!(
            json,
            serde_json::json!([{"id": 0, "title": "quotes \" and newline\n", "done": true}])
        );
    }

    #[test]
    fn deserialize_preserves_ids() {
        let mut l = TodoList::default();
        l.add("removed");
        let id = l.add("kept");
        l.remove(0).unwrap();
        let mut back = TodoList::deserialize(&l.serialize().unwrap()).unwrap();
        assert_eq!(back.tasks(), l.tasks());
        assert_eq!(back.add("next"), id + 1);
    }

    #[test]
    fn empty_json_list() {
        assert_eq!(TodoList::default().serialize().unwrap(), "[]");
        let mut l = TodoList::deserialize("[]").unwrap();
        assert!(l.tasks().is_empty());
        assert_eq!(l.add("first"), 0);
    }

    #[test]
    fn invalid_json_is_rejected() {
        for input in ["", "[ ] legacy task", "{}", "[{\"id\": 0}]", "null"] {
            assert!(TodoList::deserialize(input).is_err(), "{input}");
        }
    }

    #[test]
    fn exhausted_ids_are_rejected() {
        assert!(
            TodoList::deserialize(r#"[{"id":4294967295,"title":"last","done":false}]"#).is_err()
        );
    }
}
