pub fn show_help() {
    println!(
        r#"
Usage:
# Adding a new task
rtask add "Buy groceries"
# Output: Task added successfully (ID: 1)

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
