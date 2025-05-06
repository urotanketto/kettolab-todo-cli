use crate::manager::TaskManager;

pub fn run(manager: &TaskManager) {
    manager.list_tasks();
}
