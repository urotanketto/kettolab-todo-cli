use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

use crate::task::Task;

pub struct TaskManager {
    pub tasks: Vec<Task>,
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
            title: title.into(),
            completed: false,
        };
        self.tasks.push(task);
    }

    pub fn format_tasks(&self) -> String {
        self.tasks
            .iter()
            .map(|task| {
                if task.completed {
                    format!("[x] {}: {}", task.id, task.title)
                } else {
                    format!("[ ] {}: {}", task.id, task.title)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn list_tasks(&self) {
        println!("{}", self.format_tasks());
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
            task.title = new_title.into();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TaskManager;
    use crate::task::Task;

    #[test]
    fn test_save_and_load_tasks_with_tempfile() {
        use tempfile::NamedTempFile;

        // Create a temporary file
        let file = NamedTempFile::new().expect("Failed to create temp file");
        let path = file.path();

        // Save tasks to the temp file
        let mut manager = TaskManager::new();
        manager.add_task("Write tests");
        manager.add_task("Review PR");
        manager.save_to_file(path).expect("Failed to save tasks");

        // Load tasks from the same temp file
        let loaded = TaskManager::load_from_file(path).expect("Failed to load tasks");

        // Verify contents
        assert_eq!(loaded.tasks.len(), 2);
        assert_eq!(loaded.tasks[0].title, "Write tests");
        assert_eq!(loaded.tasks[1].title, "Review PR");
    }

    #[test]
    fn test_add_task() {
        // Arrange: Set up test data
        let task_title = "Test task";
        let mut manager = TaskManager::new();

        // Add a task
        manager.add_task(task_title);

        // Verify the task was added
        assert_eq!(manager.tasks.len(), 1);

        let task = &manager.tasks[0];
        assert_eq!(task.title, task_title);
        assert_eq!(task.id, 1);
        assert!(!task.completed);
    }

    #[test]
    fn test_format_tasks() {
        let manager = TaskManager {
            tasks: vec![
                Task {
                    id: 1,
                    title: "First task".into(),
                    completed: false,
                },
                Task {
                    id: 2,
                    title: "Second task".into(),
                    completed: true,
                },
            ],
        };

        let output = manager.format_tasks();

        let expected = "[ ] 1: First task\n[x] 2: Second task";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_mark_done() {
        let mut manager = TaskManager {
            tasks: vec![Task {
                id: 1,
                title: "Do homework".into(),
                completed: false,
            }],
        };

        // Case 1: mark existing task as done
        let result = manager.mark_done(1);
        assert!(result, "Expected mark_done to return true for existing ID");
        assert!(
            manager.tasks[0].completed,
            "Expected task to be marked as completed"
        );

        // Case 2: attempt to mark non-existent task
        let result = manager.mark_done(2);
        assert!(
            !result,
            "Expected mark_done to return false for non-existent ID"
        );
    }

    #[test]
    fn test_update_task() {
        let mut manager = TaskManager {
            tasks: vec![Task {
                id: 1,
                title: "Old Title".into(),
                completed: false,
            }],
        };

        //Case 1: update existing task
        let updated = manager.update_task(1, "New Title");
        assert!(
            updated,
            "Expected update_task to return true for existing ID"
        );
        assert_eq!(manager.tasks[0].title, "New Title");

        // Case 2: update non-existent task
        let updated = manager.update_task(2, "Should Not Work");
        assert!(
            !updated,
            "Expected update_task to return false for non-existent ID",
        );
    }

    #[test]
    fn test_get_task_mut_found() {
        let mut manager = TaskManager {
            tasks: vec![
                Task {
                    id: 1,
                    title: "Task 1".into(),
                    completed: false,
                },
                Task {
                    id: 2,
                    title: "Task 2".into(),
                    completed: false,
                },
            ],
        };

        // Test: Should find the task with ID 2
        if let Some(task) = manager.get_task_mut(2) {
            assert_eq!(task.title, "Task 2");

            // Modify the task to ensure the mutable reference works
            task.completed = true;
        } else {
            panic!("Expected to find task with ID 2");
        }

        // Confirm that the modification is reflected in the original structure
        let updated_task = manager.tasks.iter().find(|t| t.id == 2).unwrap();
        assert!(updated_task.completed);
    }

    #[test]
    fn test_get_task_mut_not_found() {
        let mut manager = TaskManager {
            tasks: vec![Task {
                id: 1,
                title: "Task 1".into(),
                completed: false,
            }],
        };

        // Test: Sourl return None for a nonexistent ID
        let task = manager.get_task_mut(99);
        assert!(task.is_none());
    }
}
