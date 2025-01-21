use crate::{
    command::{
        CommandAdd, CommandDelete, CommandList, CommandMarkDone, CommandMarkInProgress,
        CommandUpdate,
    },
    db::TaskStorage,
    task::{Task, TaskId, TaskStatus},
    Error, Result,
};

pub struct UpdateStatusResult {
    pub task_id: TaskId,
    pub old_status: TaskStatus,
    pub new_status: TaskStatus,
}

pub struct CommandExecutor {
    db: Box<dyn TaskStorage>,
}

impl CommandExecutor {
    pub fn new(db: Box<dyn TaskStorage>) -> Self {
        Self { db }
    }
}

impl CommandExecutor {
    pub fn execute_command_add(&mut self, command: CommandAdd) -> Result<Task> {
        let id = TaskId::new(1 + self.db.count_tasks())?;
        let description = command.description;
        let task = Task::new(id, description);
        self.db.create_task(task.clone())?;
        Ok(task)
    }

    pub fn execute_command_update(&mut self, command: CommandUpdate) -> Result<Task> {
        let task = self.db.get_task(&command.id).ok_or(Error::TaskNotFound {
            id: command.id.to_string(),
        })?;
        let mut updated_task = task.clone();
        updated_task.set_description(command.description);
        self.db.update_task(&command.id, updated_task.clone())?;
        Ok(updated_task)
    }

    pub fn execute_command_delete(&mut self, command: CommandDelete) -> Result<TaskId> {
        self.db.delete_task(&command.id)?;
        Ok(command.id)
    }

    pub fn execute_command_mark_in_progress(
        &mut self,
        command: CommandMarkInProgress,
    ) -> Result<UpdateStatusResult> {
        let task = self.db.get_task(&command.id).ok_or(Error::TaskNotFound {
            id: command.id.to_string(),
        })?;
        let task_id = task.id;
        let old_status = task.status.clone();
        let mut updated_task = task.clone();
        updated_task.set_status(TaskStatus::InProgress);
        self.db.update_task(&command.id, updated_task)?;
        Ok(UpdateStatusResult {
            task_id,
            old_status,
            new_status: TaskStatus::InProgress,
        })
    }

    pub fn execute_command_mark_done(&mut self, command: CommandMarkDone) -> Result<UpdateStatusResult> {
        let task = self.db.get_task(&command.id).ok_or(Error::TaskNotFound {
            id: command.id.to_string(),
        })?;
        let task_id = task.id;
        let old_status = task.status.clone();
        let mut updated_task = task.clone();
        updated_task.set_status(TaskStatus::Done);
        self.db.update_task(&command.id, updated_task)?;
        Ok(UpdateStatusResult {
            task_id,
            old_status,
            new_status: TaskStatus::Done,
        })
    }

    pub fn execute_command_list(&self, command: CommandList) -> Result<Vec<&Task>> {
        let tasks: Vec<&Task> = self
            .db
            .get_tasks()
            .iter()
            .filter(|task| {
                command
                    .status
                    .as_ref()
                    .is_none_or(|status| *status == task.status)
            })
            .collect();
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use crate::task::TaskDescription;

    use super::*;

    struct MockDb {
        tasks: Vec<Task>,
    }
    impl MockDb {
        fn new() -> Self {
            Self { tasks: vec![] }
        }
    }
    impl TaskStorage for MockDb {
        fn create_task(&mut self, task: Task) -> Result<()> {
            self.tasks.push(task);
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
            if let Some(t) = self.tasks.iter_mut().find(|task| task.id == *id) {
                *t = task;
            }
            Ok(())
        }

        fn delete_task(&mut self, id: &TaskId) -> Result<()> {
            self.tasks = self
                .tasks
                .iter()
                .cloned()
                .filter(|task| task.id != *id)
                .collect();
            Ok(())
        }
    }

    #[test]
    fn test_execute_command_add() {
        let db = Box::new(MockDb::new());
        let mut command_executor = CommandExecutor::new(db);
        let command: CommandAdd = CommandAdd {
            description: TaskDescription::new("walk the dog".to_string()).unwrap(),
        };
        let result = command_executor.execute_command_add(command);
        assert!(result.is_ok());
        let result = command_executor.execute_command_list(CommandList { status: None });
        assert!(result.is_ok());
    }
}
