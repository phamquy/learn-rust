use std::{env, fs, process};
use p2_todo_cli::TodoList;

fn main() {

    let path = std::path::PathBuf::from(
    std::env::var_os("HOME").unwrap_or_else(|| {
        eprintln!("HOME environment variable is not set");
        std::process::exit(1);
    }),
    )
    .join(".todo");
    let mut args = env::args().skip(1);
    let command = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Missing command");
            std::process::exit(1);
        }
    };

    // Check if ~/.todolist exists and create it if it doesn't
    if !path.exists() {
        if let Err(e) = fs::File::create(&path) {
            eprintln!("Failed to create todo file: {}", e);
            std::process::exit(1);
        }
    }

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => process::exit(1),
    };

    // Deserialize the contents into a TodoList, handling any errors if necessary
    let mut todo_list = TodoList::deserialize(&contents).unwrap_or_else(|e| {
        eprintln!("Failed to deserialize todo list: {}", e);
        std::process::exit(1);
    });

    match command.as_str() {
        "add" => {
            let title = match args.next() {
                Some(arg) => arg,
                None => {
                    eprintln!("Missing title for add command");
                    std::process::exit(1);
                }
            };
            todo_list.add(title.as_str());
            let serialized = todo_list.serialize();
            if let Err(e) = fs::write(&path, serialized) {
                eprintln!("Failed to write todo file: {}", e);
                std::process::exit(1);
            }
        },
        "list" => {
            for t in todo_list.tasks(){
                println!("{}. [{}] {}", t.id, if t.done { "x" } else { " " }, t.title);
            }
        },
        "done" => {
            let index = match args.next() {
                Some(arg) => arg.parse::<u32>().unwrap_or_else(|_| {
                    eprintln!("Invalid index for done command");
                    std::process::exit(1);
                }),
                None => {
                    eprintln!("Missing index for done command");
                    std::process::exit(1);
                }
            };
            todo_list.complete(index);
            let serialized = todo_list.serialize();
            if let Err(e) = fs::write(&path, serialized) {
                eprintln!("Failed to write todo file: {}", e);
                std::process::exit(1);
            }
        },
        "rm" => {
            let index = match args.next() {
                Some(arg) => arg.parse::<u32>().unwrap_or_else(|_| {
                    eprintln!("Invalid index for rm command");
                    std::process::exit(1);
                }),
                None => {
                    eprintln!("Missing index for rm command");
                    std::process::exit(1);
                }
            };
            // Handle the "rm" command
            todo_list.remove(index);
            let serialized = todo_list.serialize();
            if let Err(e) = fs::write(&path, serialized) {
                eprintln!("Failed to write todo file: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
