use crate::{command::{CommandAdd, CommandList, CommandMarkDone, CommandMarkInProgress}, db::Db, task::{Task, TaskId, TaskStatus}, Error, Result};

pub fn execute_command_add(command: CommandAdd) -> Result<()> {
    let mut db = Db::new()?;
    let id = TaskId::new(1 + db.count_tasks())?;
    let description = command.description;
    let task = Task::new(id, description);
    db.create_task(task)?;
    Ok(())
}

pub fn execute_command_mark_in_progress(command: CommandMarkInProgress) -> Result<()> {
    let mut db = Db::new()?;
    let task = db.get_task(&command.id)
        .ok_or(Error::TaskNotFound { id: command.id.to_string() })?;
    let mut updated_task = task.clone();
    updated_task.status = TaskStatus::InProgress;
    db.update_task(command.id, updated_task)?;
    Ok(())
}

pub fn execute_command_mark_done(command: CommandMarkDone) -> Result<()> {
    let mut db = Db::new()?;
    let task = db.get_task(&command.id)
        .ok_or(Error::TaskNotFound { id: command.id.to_string() })?;
    let mut updated_task = task.clone();
    updated_task.status = TaskStatus::Done;
    db.update_task(command.id, updated_task)?;
    Ok(())
}

pub fn execute_command_list(command: CommandList) -> Result<()> {
    let db = Db::new()?;
    db.tasks.iter()
        .filter(|task|
            command.status.as_ref().is_none_or(|status| *status == task.status)
        )
        .for_each(|task| {
            println!("{}", task);
        });
    Ok(())
}
