# Task Tracker

CLI app to track tasks

App saves tasks into `$HOME/.roadmap-task-tracker.json`

## Details
Do not use non-unicode characters (didn't want to add extra deps to handle these rare use cases).

Ideally I should use no libs, but while languages like JS have native Date and JSON handling,
Rust, by design, doesn't. Thus I allowed myself to rely on crates like serde and chrono.

Trying to use as few crates as possible is the reason I opt not to use [clap](https://crates.io/crates/clap).

## Usage
After building:
```bash
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
```


## Test
```bash
cargo test

```

