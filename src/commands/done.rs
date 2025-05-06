use crate::manager::TaskManager;

pub fn run(manager: &mut TaskManager, args: &[String], data_path: &str) {
    if args.len() < 1 {
        eprintln!("Please provide the task ID to mark as done.");
        return;
    }

    match args[0].parse::<u32>() {
        Ok(id) => {
            if manager.mark_done(id) {
                manager.save_to_file(data_path).unwrap();
                println!("Task {} marked as done.", id);
            } else {
                eprintln!("Task with ID {} not found.", id);
            }
        }
        Err(_) => {
            eprintln!("Invalid task ID: {}", args[0])
        }
    }
}
