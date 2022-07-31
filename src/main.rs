use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use components::{list::TaskList, status::Status, task::Task};
use home::home_dir;

use crate::{
    components::{
        category::Category,
        outcome::Outcome,
        short_string::ShortString,
        time::{date_specifier::DateSpecifier, duration::Throwback},
        TaskId,
    },
    format::TaskListFormatter,
};

pub(crate) mod components;
pub(crate) mod format;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about=None)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Add a new task to the to-do list.
    Add {
        /// Short description of the task, must be <= 50 chars long.
        short: ShortString,

        /// Optional long description of the task.
        #[clap(short, long)]
        long: Option<String>,

        /// Task category.
        #[clap(arg_enum, short, long)]
        category: Option<Category>,

        /// Task deadline, e.g. today, tomorrow, this/tuesday, next/friday or 01/08/2022.
        #[clap(short, long)]
        deadline: Option<DateSpecifier>,
    },

    /// Set the status of a task.
    Start {
        /// ID of task to set.
        id: TaskId,
    },

    /// Update a task.
    Update {
        /// Task ID.
        id: TaskId,

        /// What to update.
        #[clap(subcommand)]
        update: Updates,
    },

    /// Print misc. summaries.
    Print {
        #[clap(short, long, arg_enum, default_value_t = TaskListFormatter::Long)]
        format: TaskListFormatter,

        #[clap(subcommand)]
        summary: Summaries,
    },

    /// Deletes all tasks.
    Clear,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Updates {
    /// Updates the tasks's short description.
    Short {
        short: ShortString,
    },

    /// Updates the tasks's long description.
    Long {
        long: String,
    },

    /// Removes the task's long description.
    DiscardLong,

    /// Updates the tasks's category.
    Category {
        #[clap(arg_enum)]
        category: Category,
    },

    /// Updates the tasks's status.
    Status {
        #[clap(subcommand)]
        status: Statuses,
    },

    /// Updates the tasks's deadline.
    Deadline {
        deadline: DateSpecifier,
    },

    /// Removes the task's long description.
    DiscardDeadline,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Statuses {
    /// Task is in progress.
    InProgress,

    /// Task is blocked by another task.
    BlockedByTask { id: TaskId },

    /// Task is blocked for some reason.
    BlockedByOther { reason: ShortString },

    /// Task has been completed.
    Completed {
        #[clap(arg_enum)]
        outcome: Outcome,
    },

    /// Task has been discarded for some reason.
    Discarded { reason: ShortString },
}

#[derive(Debug, Subcommand)]
pub(crate) enum Summaries {
    /// Show all items still pending.
    Pending,

    /// Show all items completed in the past duration.
    Completed {
        /// Time interval to go back.
        past: Throwback,
    },
}

fn get_filepath() -> PathBuf {
    if let Ok(s) = std::env::var("WWIDA_CACHE") {
        let filepath = PathBuf::from_str(&s).unwrap();
        std::fs::create_dir_all(filepath.parent().expect("wwida cache file has no parent directory"))
            .expect("could not create wwida cache file parent directory");
        filepath
    } else {
        let mut filepath = home_dir().expect("no home directory found");
        filepath.push(".wwida_cache");
        filepath
    }
}

fn main() {
    let cli = Cli::parse();
    
    let filepath = get_filepath();
    let mut tasks = TaskList::load(&filepath).expect("could not load tasks");
    match cli.command {
        Commands::Add {
            short,
            long,
            category,
            deadline,
        } => {
            let task = Task::new(short, category, long, deadline).expect("could not create new task");
            tasks.add_task(task);
        },
        Commands::Print { format, summary } => {
            match summary {
                Summaries::Pending => {
                    let iter = tasks
                        .iter()
                        .filter(|(_, task)| !matches!(task.last_status(), Status::Completed { .. }));
                    format.print(iter);
                }
                Summaries::Completed { past } => {
                    let iter = tasks.iter().filter(|(_, task)| {
                        if let Status::Completed { date, .. } = task.last_status() {
                            past.contains(date)
                        } else {
                            false
                        }
                    });
                    format.print(iter);
                }
            };
        },
        Commands::Start { id } => tasks.get_task_mut(id).expect("task ID not found").start().unwrap(),
        Commands::Update { id, update } => {
            match update {
                Updates::Short { short } => tasks.get_task_mut(id).expect("task ID not found").set_short(short),
                Updates::Long { long } => tasks.get_task_mut(id).expect("task ID not found").set_long(long),
                Updates::Category { category } => tasks.get_task_mut(id).expect("task ID not found").set_category(category),
                Updates::Status { status } => {
                    let status = match status {
                        Statuses::InProgress => Status::in_progress(),
                        Statuses::BlockedByTask { id: blocking_id } => Status::blocked_by_task(blocking_id, &tasks).expect("could not set status"),
                        Statuses::BlockedByOther { reason } => Status::blocked_by_other(reason),
                        Statuses::Completed { outcome } => Status::completed(outcome),
                        Statuses::Discarded { reason } => Status::discarded(reason),
                    };
                    tasks.get_task_mut(id).expect("task ID not found").set_status(status);
                },
                Updates::Deadline { deadline } => tasks.get_task_mut(id).expect("task ID not found").set_deadline(deadline.into()),
                Updates::DiscardLong => tasks.get_task_mut(id).expect("task ID not found").unset_long(),
                Updates::DiscardDeadline => tasks.get_task_mut(id).expect("task ID not found").unset_deadline(),
            }
        },
        Commands::Clear => {
            let num = tasks.num_tasks();
            tasks.clear();
            println!("CLEARED {num} TASKS")
        },
    }

    tasks.save(&filepath).expect("could not save tasks");
}
