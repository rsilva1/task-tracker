/* Project Constraints
* Use the native file system module of your programming language to interact with the JSON file.
* No external libraries nor frameworks
* As Rust doesn't handle JSON naturally as JS, I'll "cheat" into using serde
* For the sake of simplicity, do not try to use invalid Unicode characters
*/

mod error;
use cli_commands::show_help;
use command::{parse_command, Command};
pub use error::{Error, Result};
use execute_command::{execute_command_add, execute_command_list, execute_command_mark_done, execute_command_mark_in_progress};

mod task;
mod command;
mod execute_command;
mod cli_commands;
mod db;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        show_help();
        return;
    }

    let command = parse_command(args).unwrap();
    println!("Command is {:#?}", command);

    let result = match command {
        Command::CommandAdd(command_add) => execute_command_add(command_add),
        Command::CommandUpdate(command_update) => todo!(),
        Command::CommandDelete(command_delete) => todo!(),
        Command::CommandMarkInProgress(command_mark_in_progress) => execute_command_mark_in_progress(command_mark_in_progress),
        Command::CommandMarkDone(command_mark_done) => execute_command_mark_done(command_mark_done),
        Command::CommandList(command_list) => execute_command_list(command_list),
    };

    if let Err(e) = result {
        println!("Error: {}", e);
    }

    ()
}

