use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Task {
    pub fn new(id: TaskId, description: TaskDescription) -> Self {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        // todo: handle this "rare"? error
        let now = now.unwrap_or_default().as_secs();
        Task {
            id,
            description,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskId {
    id: u32,
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl TaskId {
    pub fn new_from_string(id: String) -> Result<Self> {
        let id = id.parse::<u32>().map_err(|_| Error::IdMustBeNumber { id })?;
        return Ok(Self { id })
    }

    pub fn new(id: u32) -> Result<Self> {
        return Ok(Self { id })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDescription {
    description: String,
}

impl std::fmt::Display for TaskDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl TaskDescription {
    pub fn new(description: String) -> Result<Self> {
        if description.is_empty() {
            return Err(Error::EmptyDescription)
        }
        Ok(Self { description })
    }
}
