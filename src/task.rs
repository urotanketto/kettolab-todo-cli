use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_serialization_roundtrip() {
        let original_task = Task {
            id: 1,
            title: "Write tests".into(),
            completed: false,
        };

        // Serialize the task to a JSON string
        let json = serde_json::to_string(&original_task).expect("Serialization failed");

        // Deserialize the JSON string back into a Task
        let deserialized: Task = serde_json::from_str(&json).expect("Deserialization failed");

        // Assert that the original and deserialized tasks are equal
        assert_eq!(original_task.id, deserialized.id);
        assert_eq!(original_task.title, deserialized.title);
        assert_eq!(original_task.completed, deserialized.completed);
    }
}
