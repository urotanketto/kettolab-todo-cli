use std::env;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, title: &str) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task {
            id,
            title: title.to_string(),
            completed: false,
        };
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                task.id,
                task.title,
                if task.completed { "done" } else { "pending" }
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut manager = TaskManager::new();

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
            manager.add_task(&title);
            println!("Task added.");
        }
        "list" => {
            manager.list_tasks();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

