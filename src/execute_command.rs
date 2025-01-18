use crate::{command::CommandAdd, db::Db, task::{Task, TaskId}, Result};

pub fn execute_command_add(command: CommandAdd) -> Result<()> {
    let mut db = Db::new()?;
    let id = TaskId::new(1 + db.count_tasks())?;
    let description = command.description;
    let task = Task::new(id, description);
    db.save_task(task)?;
    Ok(())
}
