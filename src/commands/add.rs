use crate::manager::TaskManager;

pub fn run(manager: &mut TaskManager, args: &[String], data_path: &str) {
    if args.is_empty() {
        eprintln!("Please provide a task description.");
        return;
    }
    let title = args.join(" ");
    manager.add_task(&title);
    manager.save_to_file(data_path).unwrap();
    println!("Task added.");
}
