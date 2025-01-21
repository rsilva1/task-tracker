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
        Command::CommandAdd(command_add) => command_executor
            .execute_command_add(command_add)
            .inspect(|r| show_added_task(r))
            .map(|_| ()),
        Command::CommandUpdate(command_update) => command_executor
            .execute_command_update(command_update)
            .inspect(|r| show_updated_task(r))
            .map(|_| ()),
        Command::CommandDelete(command_delete) => command_executor
            .execute_command_delete(command_delete)
            .inspect(|r| show_deleted_task(r))
            .map(|_| ()),
        Command::CommandMarkInProgress(command_mark_in_progress) => command_executor
            .execute_command_mark_in_progress(command_mark_in_progress)
            .inspect(|r| show_updated_status(r))
            .map(|_| ()),
        Command::CommandMarkDone(command_mark_done) => command_executor
            .execute_command_mark_done(command_mark_done)
            .inspect(|r| show_updated_status(r))
            .map(|_| ()),
        Command::CommandList(command_list) => command_executor
            .execute_command_list(command_list)
            .inspect(|r| show_tasks(r))
            .map(|_| ()),
    };
    result?;

    Ok(())
}
