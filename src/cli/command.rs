use std::collections::BTreeSet;

use clap::Subcommand;
use itertools::Itertools;

use crate::{
    components::{
        category::Category, short_string::ShortString, task::Task, tasks::Tasks, time::date_specifier::DateSpecifier,
        TaskId, priority::Priority, tag::{tags_from_comma_separated_string, Tag},
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

        #[clap(value_enum, short, long, default_value_t)]
        category: Category,

        /// Task deadline, e.g. today, tomorrow, this/tuesday, next/friday or 01/08/2022.
        #[clap(short, long)]
        deadline: Option<DateSpecifier>,

        #[clap(short, long, value_enum, default_value_t)]
        priority: Priority,

        /// Comma-separated list of tags.
        #[clap(short, long, default_value_t)]
        tags: String,
    },

    /// Set an unstarted task to in progress.
    Start {
        id: TaskId,
    },

    /// Update a task.
    Update {
        id: TaskId,

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

    /// Shows all currently used tags.
    Tags,
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
            Command::Tags => {
                let tags: BTreeSet<Tag> = tasks.iter().flat_map(|task| task.tags.iter()).cloned().collect();
                println!("{}", tags.iter().join(", "));
            },
        };
        Ok(())
    }
}
