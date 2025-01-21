use std::{path::PathBuf, sync::OnceLock};

use crate::{
    task::{Task, TaskId},
    Error, Result,
};

const DB_FILENAME: &str = ".roadmap-task-tracker.json";
static DB_PATH: OnceLock<PathBuf> = OnceLock::new();

fn get_db_path() -> &'static PathBuf {
    DB_PATH.get_or_init(|| {
        let mut path = home::home_dir().expect(&Error::HomePathNotFound.to_string());
        path.push(DB_FILENAME);
        path
    })
}

pub trait TaskStorage {
    fn create_task(&mut self, task: Task) -> Result<()>;
    fn count_tasks(&self) -> u32;
    fn get_task(&mut self, id: &TaskId) -> Option<&Task>;
    fn get_tasks(&self) -> &Vec<Task>;
    fn update_task(&mut self, id: &TaskId, task: Task) -> Result<()>;
    fn delete_task(&mut self, id: &TaskId) -> Result<()>;
}

pub struct Db {
    pub tasks: Vec<Task>,
}

impl Db {
    pub fn new() -> Result<Self> {
        match get_db_path().exists() {
            true => {
                let content = std::fs::read_to_string(DB_PATH.get().expect(""))
                    .map_err(|_| Error::FailedToAccessPersistedData)?;
                let tasks: Vec<Task> = serde_json::from_str(&content)
                    .map_err(|_| Error::FailedToAccessPersistedData)?;
                Ok(Self { tasks })
            }
            false => Ok(Self { tasks: vec![] }),
        }
    }
}

impl TaskStorage for Db {
    fn create_task(&mut self, task: Task) -> Result<()> {
        let tasks = &mut self.tasks;
        tasks.push(task);
        let content = serde_json::json!(tasks).to_string();
        std::fs::write(get_db_path(), content).map_err(|_| Error::FailedToPersistChanges)?;
        Ok(())
    }

    fn count_tasks(&self) -> u32 {
        self.tasks.len() as u32
    }

    fn get_task(&mut self, id: &TaskId) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == *id)
    }

    fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn update_task(&mut self, id: &TaskId, task: Task) -> Result<()> {
        let position = self
            .tasks
            .iter()
            .position(|task| task.id == *id)
            .ok_or(Error::TaskNotFound { id: id.to_string() })?;
        self.tasks[position] = task;
        let content = serde_json::json!(&self.tasks).to_string();
        std::fs::write(get_db_path(), content).map_err(|_| Error::FailedToPersistChanges)?;
        Ok(())
    }

    fn delete_task(&mut self, id: &TaskId) -> Result<()> {
        let task_count = self.tasks.len();
        self.tasks = self
            .tasks
            .iter()
            .filter(|task| task.id != *id)
            .cloned()
            .collect();
        if self.tasks.len() == task_count {
            return Err(Error::TaskNotFound { id: id.to_string() });
        }
        let content = serde_json::json!(&self.tasks).to_string();
        std::fs::write(get_db_path(), content).map_err(|_| Error::FailedToPersistChanges)?;
        Ok(())
    }
}
