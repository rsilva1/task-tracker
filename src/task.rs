use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl TaskStatus {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "todo" => Ok(TaskStatus::Todo),
            "in_progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            _ => Err(Error::UnknownStatus {
                status: s.to_string(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Task {
    pub fn new(id: TaskId, description: TaskDescription) -> Self {
        let now = Local::now();
        Task {
            id,
            description,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_description(&mut self, description: TaskDescription) {
        self.description = description;
        self.updated_at = Local::now();
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Local::now();
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
Task: {}
Description: {}
Status: {}
Created At: {}
Updated At: {}
"#,
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
        let id = id
            .parse::<u32>()
            .map_err(|_| Error::IdMustBeNumber { id })?;
        return Ok(Self { id });
    }

    pub fn new(id: u32) -> Result<Self> {
        return Ok(Self { id });
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
            return Err(Error::EmptyDescription);
        }
        Ok(Self { description })
    }
}
