use std::env;

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks: Vec<Task> = Vec::new();

    if args.len() < 2 {
        eprintln!("Usage: todo <add|list> [task description]");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Please provide a task description.");
                return;
            }
            let title = args[2..].join(" ");
            let task = Task {
                id: tasks.len() + 1,
                title,
            };
            tasks.push(task);
            println!("Task added.");
        }
        "list" => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in &tasks {
                    println!("[{}] {}", task.id, task.title);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

