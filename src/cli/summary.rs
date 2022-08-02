use std::collections::HashSet;

use clap::Subcommand;

use crate::{
    components::{time::duration::TimeInterval, tag::Tag, task::Task, TaskId, tasks::Tasks},
    format::TaskListFormatter,
};

use super::sort::SortBy;

#[derive(Debug, Subcommand)]
pub(crate) enum Summary {
    /// Show all items still pending.
    Pending,

    /// Show all items completed in the past duration.
    Completed {
        /// Time interval to go back.
        interval: TimeInterval,
    },
}

impl Summary {
    pub(crate) fn run(self, format: TaskListFormatter, tasks: &Tasks, filter: Option<Vec<Tag>>, sort: SortBy) -> anyhow::Result<()> {
        let filter: Option<HashSet<Tag>> = filter.map(|v| v.into_iter().collect());

        let mut tasks = tasks.id_task_iter()
            .filter(|(_, task)| if let Some(filter) = &filter { !filter.is_disjoint(&task.tags) } else { true })
            .filter(|(_, task)| match &self {
                Summary::Pending => !task.is_completed(),
                Summary::Completed { interval } => task.is_completed_in_past(interval),
            })
            .collect::<Vec<(TaskId, &Task)>>();

        match sort {
            SortBy::Id => (),  // already sorted by ID
            SortBy::Category => tasks.sort_unstable_by(|(_, ta), (_, tb)| ta.category.cmp(&tb.category)),
            SortBy::Priority => tasks.sort_unstable_by(|(_, ta), (_, tb)| tb.priority.cmp(&ta.priority)),
            SortBy::Deadline => tasks.sort_unstable_by(|(_, ta), (_, tb)| ta.deadline.cmp(&tb.deadline)),
        }

        format.print(tasks);
        Ok(())
    }
}
