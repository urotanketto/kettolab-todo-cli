use crate::manager::TaskManager;
use std::io::{self, Write};

pub fn run(manager: &mut TaskManager, args: &[String], data_path: &str) {
    if args.len() < 1 {
        eprintln!("Please provide the task ID.");
        return;
    }

    let id: u32 = match args[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid ID: {}", args[0]);
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
