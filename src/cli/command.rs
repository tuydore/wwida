use clap::Subcommand;

use crate::{
    components::{
        category::Category, short_string::ShortString, task::Task, tasks::Tasks, time::date_specifier::DateSpecifier,
        TaskId, priority::Priority, tag::tags_from_comma_separated_string,
    },
    format::TaskListFormatter,
};

use super::{summary::Summary, update::Update, sort::SortBy};

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
        #[clap(value_enum, short, long, default_value_t)]
        category: Category,

        /// Task deadline, e.g. today, tomorrow, this/tuesday, next/friday or 01/08/2022.
        #[clap(short, long)]
        deadline: Option<DateSpecifier>,

        #[clap(short, long, value_enum, default_value_t)]
        priority: Priority,

        /// Comma-separated list of tags.
        #[clap(short, long)]
        tags: String,
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
        #[clap(long, value_enum, default_value_t)]
        format: TaskListFormatter,

        /// Comma-separated list of tags.
        #[clap(long)]
        filter: Option<String>,

        #[clap(subcommand)]
        summary: Summary,

        #[clap(short, long, value_enum, default_value_t)]
        sort: SortBy,
    },

    /// Deletes all tasks.
    Clear,
}

impl Command {
    pub(crate) fn run(self, tasks: &mut Tasks) -> anyhow::Result<()> {
        match self {
            Command::Add {
                short,
                long,
                category,
                deadline,
                priority,
                tags,
                
            } => {
                let task = Task::new(short, category, long, deadline, priority, tags_from_comma_separated_string(tags)?)?;
                tasks.add_task(task);
            }
            Command::Start { id } => tasks.get_task_mut_err(id)?.start()?,
            Command::Update { id, update } => update.run(id, tasks)?,
            Command::Print { format, summary, filter, sort } => summary.run(format, tasks, filter.map(tags_from_comma_separated_string).transpose()?, sort)?,
            Command::Clear => {
                let num = tasks.num_tasks();
                tasks.clear();
                println!("Cleared {num} tasks.")
            }
        };
        Ok(())
    }
}
