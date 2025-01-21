use crate::{
    execute_command::UpdateStatusResult,
    task::{Task, TaskId},
};

pub fn show_tasks(tasks: &Vec<&Task>) {
    tasks.iter().for_each(|task| println!("{}", task));
}

pub fn show_added_task(task: &Task) {
    println!(
        r#"Added Task:
{task}
    "#
    );
}

pub fn show_updated_task(task: &Task) {
    println!(
        r#"Updated Task:
{task}
    "#
    );
}

pub fn show_deleted_task(r: &TaskId) {
    println!("Successfully Deleted Task {}", r);
}

pub fn show_updated_status(r: &UpdateStatusResult) {
    println!(
        r#"Task Id: {}
Previous status: {}
New status: {}
"#,
        r.task_id, r.old_status, r.new_status
    );
}

pub fn show_help() {
    println!(
        r#"Usage:
# Adding a new task
rtask add "Buy groceries"

# Updating and deleting tasks
rtask update 1 "Buy groceries and cook dinner"
rtask delete 1

# Marking a task as in progress or done
rtask mark-in-progress 1
rtask mark-done 1

# Listing all tasks
rtask list

# Listing tasks by status
rtask list done
rtask list todo
rtask list in-progress
"#
    );
}
