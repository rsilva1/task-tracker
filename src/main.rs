/* Project Constraints
* Use the native file system module of your programming language to interact with the JSON file.
* No external libraries nor frameworks
* As Rust doesn't handle JSON naturally as JS, I'll "cheat" into using serde
* For the sake of simplicity, do not try to use invalid Unicode characters
*/

mod error;
use command::{parse_command, Command};
pub use error::{Error, Result};
use execute_command::CommandExecutor;
use presentation::{
    show_added_task, show_deleted_task, show_help, show_tasks, show_updated_status,
    show_updated_task,
};

mod command;
mod db;
mod execute_command;
mod presentation;
mod task;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        show_help();
        return Ok(());
    }

    let command = parse_command(args)?;

    let db = Box::new(db::Db::new()?);
    let mut command_executor = CommandExecutor::new(db);

    let result = match command {
        Command::Add(command_add) => command_executor
            .execute_command_add(command_add)
            .inspect(show_added_task)
            .map(|_| ()),
        Command::Update(command_update) => command_executor
            .execute_command_update(command_update)
            .inspect(show_updated_task)
            .map(|_| ()),
        Command::Delete(command_delete) => command_executor
            .execute_command_delete(command_delete)
            .inspect(show_deleted_task)
            .map(|_| ()),
        Command::MarkInProgress(command_mark_in_progress) => command_executor
            .execute_command_mark_in_progress(command_mark_in_progress)
            .inspect(show_updated_status)
            .map(|_| ()),
        Command::MarkDone(command_mark_done) => command_executor
            .execute_command_mark_done(command_mark_done)
            .inspect(show_updated_status)
            .map(|_| ()),
        Command::List(command_list) => command_executor
            .execute_command_list(command_list)
            .inspect(show_tasks)
            .map(|_| ()),
    };
    result?;

    Ok(())
}
