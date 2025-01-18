use crate::{task::{TaskDescription, TaskId, TaskStatus}, Error, Result};

#[derive(Debug)]
pub struct CommandAdd {
    pub description: TaskDescription,
}

#[derive(Debug)]
pub struct CommandUpdate {
    pub id: TaskId,
    pub description: TaskDescription,
}

#[derive(Debug)]
pub struct CommandDelete {
    pub id: TaskId,
}

#[derive(Debug)]
pub struct CommandMarkInProgress {
    pub id: TaskId,
}

#[derive(Debug)]
pub struct CommandMarkDone {
    pub id: TaskId,
}

#[derive(Debug)]
pub struct CommandList {
    pub status: TaskStatus,
}

#[derive(Debug)]
pub enum Command {
    CommandAdd(CommandAdd),
    CommandUpdate(CommandUpdate),
    CommandDelete(CommandDelete),
    CommandMarkInProgress(CommandMarkInProgress),
    CommandMarkDone(CommandMarkDone),
}

pub fn parse_command(args: Vec<String>) -> Result<Command> {
    if args.len() == 1 {
        return Err(Error::NoCommand)
    }
    let cmd = args[1].clone();
    match cmd.as_str() {
        "add" => parse_add_command(args).map(|cmd| Command::CommandAdd(cmd)),
        "update" => parse_update_command(args).map(|cmd| Command::CommandUpdate(cmd)),
        "delete" => parse_delete_command(args).map(|cmd| Command::CommandDelete(cmd)),
        "mark-in-progress" => parse_mark_in_progress_command(args).map(|cmd| Command::CommandMarkInProgress(cmd)),
        "mark-done" => parse_mark_done_command(args).map(|cmd| Command::CommandMarkDone(cmd)),
        _ => Err(Error::UnknownCommand{ command: cmd })
    }
}

fn parse_add_command(args: Vec<String>) -> Result<CommandAdd> {
    validate_args_length(&args, 3)?;
    let description = TaskDescription::new(args[2].clone())?;
    Ok(CommandAdd { description })
}

fn parse_update_command(args: Vec<String>) -> Result<CommandUpdate> {
    validate_args_length(&args, 4)?;
    let id = TaskId::new_from_string(args[2].clone())?;
    let description = TaskDescription::new(args[3].clone())?;
    Ok(CommandUpdate { id, description })
}

fn parse_delete_command(args: Vec<String>) -> Result<CommandDelete> {
    validate_args_length(&args, 3)?;
    let id = TaskId::new_from_string(args[2].clone())?;
    Ok(CommandDelete { id })
}

fn parse_mark_in_progress_command(args: Vec<String>) -> Result<CommandMarkInProgress> {
    validate_args_length(&args, 3)?;
    let id = TaskId::new_from_string(args[2].clone())?;
    Ok(CommandMarkInProgress { id })
}

fn parse_mark_done_command(args: Vec<String>) -> Result<CommandMarkDone> {
    validate_args_length(&args, 3)?;
    let id = TaskId::new_from_string(args[2].clone())?;
    Ok(CommandMarkDone { id })
}

fn validate_args_length(args: &Vec<String>, expected: u8) -> Result<()> {
    if args.len() != (expected as usize) {
        return Err(Error::WrongNumberOfArguments {
            expected,
            got: args.len() as u8,
        })
    }
    Ok(())
}
