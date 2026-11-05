//! Topic: structs, enums, pattern matching.

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Push(i32),
    Pop,
    Add,
}

/// Parse "push 3", "pop", "add". Anything else (including a bad number) is None.
pub fn parse_command(input: &str) -> Option<Command> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas() {
        assert!((Shape::Circle(1.0).area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(Shape::Rect { w: 2.0, h: 3.0 }.area(), 6.0);
    }

    #[test]
    fn parsing() {
        assert_eq!(parse_command("push 3"), Some(Command::Push(3)));
        assert_eq!(parse_command("pop"), Some(Command::Pop));
        assert_eq!(parse_command("add"), Some(Command::Add));
        assert_eq!(parse_command("push x"), None);
        assert_eq!(parse_command("nope"), None);
    }
}
