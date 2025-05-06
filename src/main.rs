use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
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

    pub fn mark_done(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            return true;
        }
        false
    }

    pub fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    pub fn update_task(&mut self, id: u32, new_title: &str) -> bool {
        if let Some(task) = self.get_task_mut(id) {
            task.title = new_title.to_string();
            true
        } else {
            false
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
        "done" => {
            if args.len() < 3 {
                eprintln!("Please provide the task ID to mark as done.");
                return;
            }

            match args[2].parse::<u32>() {
                Ok(id) => {
                    if manager.mark_done(id) {
                        manager.save_to_file(data_path).unwrap();
                        println!("Task {} marked as done.", id)
                    } else {
                        eprintln!("Task with ID {} not found.", id);
                    }
                }
                Err(_) => {
                    eprintln!("Invalid task ID: {}", args[2])
                }
            }
        }
        "update" => {
            if args.len() < 3 {
                eprintln!("Please provide the task ID.");
                return;
            }
            let id: u32 = match args[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid ID: {}", args[2]);
                    return;
                }
            };

            if let Some(task) = manager.get_task_mut(id) {
                println!("Update \"{}\" to", task.title);
                print!("> ");
                io::stdout().flush().unwrap();

                let mut new_title = String::new();
                if io::stdin().read_line(&mut new_title).is_ok() {
                    let new_title = new_title.trim();
                    if !new_title.is_empty() {
                        if manager.update_task(id, new_title) {
                            manager.save_to_file(data_path).unwrap();
                            println!("Task updated.");
                        } else {
                            println!("Task ID {} not found.", id);
                        }
                    } else {
                        println!("No new title entered. Task not updated.");
                    }
                } else {
                    eprintln!("Failed to read input.");
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
