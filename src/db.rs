use std::path::Path;

use crate::{task::{Task, TaskId}, Error, Result};

const DB_PATH: &str = ".roadmap-task-tracker.json";

pub struct Db {
    content: String,
    pub tasks: Vec<Task>,
}

impl Db {
    pub fn new() -> Result<Self> {
        match Path::new(DB_PATH).exists() {
            true => {
                let content = std::fs::read_to_string(DB_PATH)
                    .map_err(|_| Error::FailedToAccessPersistedData)?;
                let tasks: Vec<Task> = serde_json::from_str(&content)
                    .map_err(|_| Error::FailedToAccessPersistedData)?;
                Ok(Self { content, tasks })
            }
            false => Ok(Self {
                content: "".to_string(),
                tasks: vec![],
            })
        }
    }

    pub fn create_task(&mut self, task: Task) -> Result<()> {
        let tasks = &mut self.tasks;
        tasks.push(task);
        let content = serde_json::json!(tasks).to_string();
        std::fs::write(DB_PATH, content).map_err(|_| Error::FailedToPersistChanges)?;
        Ok(())
    }

    pub fn count_tasks(&self) -> u32 {
        self.tasks.len() as u32
    }

    pub fn get_task(&mut self, id: &TaskId) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == *id)
    }

    pub fn update_task(&mut self, id: TaskId, task: Task) -> Result<()> {
        let position = self.tasks.iter()
            .position(|task| task.id == id)
            .ok_or(Error::TaskNotFound { id: id.to_string() })?;
        self.tasks[position] = task;
        let content = serde_json::json!(&self.tasks).to_string();
        std::fs::write(DB_PATH, content).map_err(|_| Error::FailedToPersistChanges)?;
        Ok(())
    }
}

