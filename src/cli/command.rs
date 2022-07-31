use clap::Subcommand;

use crate::{components::{short_string::ShortString, category::Category, time::date_specifier::DateSpecifier, TaskId, list::Tasks, task::Task}, format::TaskListFormatter};

use super::{update::Update, summary::Summary};

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
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
        update: Update,
    },

    /// Print misc. summaries.
    Print {
        #[clap(short, long, arg_enum, default_value_t = TaskListFormatter::Long)]
        format: TaskListFormatter,

        #[clap(subcommand)]
        summary: Summary,
    },

    /// Deletes all tasks.
    Clear,
}

impl Command {
    pub(crate) fn run(self, tasks: &mut Tasks) -> anyhow::Result<()> {
        match self {
            Command::Add { short, long, category, deadline } => {
                let task = Task::new(short, category, long, deadline)?;
                tasks.add_task(task);
            },
            Command::Start { id } => tasks.get_task_mut_err(id)?.start()?,
            Command::Update { id, update } => update.run(id, tasks)?,
            Command::Print { format, summary } => summary.run(format, tasks)?,
            Command::Clear => {
                let num = tasks.num_tasks();
                tasks.clear();
                println!("Cleared {num} tasks.")
            },
        };
        Ok(())
    }
}