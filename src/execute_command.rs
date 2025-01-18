use crate::{command::{CommandAdd, CommandList}, db::Db, task::{Task, TaskId}, Result};

pub fn execute_command_add(command: CommandAdd) -> Result<()> {
    let mut db = Db::new()?;
    let id = TaskId::new(1 + db.count_tasks())?;
    let description = command.description;
    let task = Task::new(id, description);
    db.save_task(task)?;
    Ok(())
}

pub fn execute_command_list(command: CommandList) -> Result<()> {
    let mut db = Db::new()?;
    for task in db.tasks.iter() {
        println!("{}", task);
    }
    Ok(())
}
