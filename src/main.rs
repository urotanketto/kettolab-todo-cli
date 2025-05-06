mod commands;
mod manager;
mod task;

use crate::commands::{add, done, list, update};
use crate::manager::TaskManager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let data_path = "tasks.json";
    let mut manager = TaskManager::load_from_file(data_path).unwrap();

    if args.is_empty() {
        eprintln!("Usage: todo <add|list|done|update> [arguments]");
        return;
    }

    let (command, command_args) = args.split_first().unwrap();

    match command.as_str() {
        "add" => add::run(&mut manager, command_args, data_path),
        "list" => list::run(&manager),
        "done" => done::run(&mut manager, command_args, data_path),
        "update" => update::run(&mut manager, command_args, data_path),
        _ => eprintln!("Unknown command: {}", command),
    }
}
