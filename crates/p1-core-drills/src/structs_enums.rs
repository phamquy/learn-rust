//! Topic: structs, enums, pattern matching.

// use crate::structs_enums::{
//     Command::{Add, Push},
//     Shape::Circle,
// };

use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r,
            Shape::Rect { w, h } => w * h,
        }
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
    let mut words = input.split_whitespace();
    let cmd = match words.next()? {
        "push" => {
            let n = words.next()?.parse::<i32>().ok()?;
            Command::Push(n)
        }
        "pop" => Command::Pop,
        "add" => Command::Add,
        _ => return None,
    };

    if words.next().is_some() {
        return None;
    }
    Some(cmd)
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
