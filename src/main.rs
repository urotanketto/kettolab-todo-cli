use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

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

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        if !path.as_ref().exists() {
            return Ok(Self::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());

        Ok(TaskManager { tasks })
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.tasks)?;
        Ok(())
    }

    pub fn add_task(&mut self, title: &str) {
        let id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;

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
    let data_path = "tasks.json";
    let mut manager = TaskManager::load_from_file(data_path).unwrap();

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
            manager.save_to_file(data_path).unwrap();
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
