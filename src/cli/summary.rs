use clap::Subcommand;

use crate::{
    components::{tasks::Tasks, time::duration::TimeInterval},
    format::TaskListFormatter,
};

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
    pub(crate) fn run(self, format: TaskListFormatter, tasks: &mut Tasks) -> anyhow::Result<()> {
        match self {
            Summary::Pending => {
                format.print(tasks.iter().filter(|(_, task)| !task.is_completed()));
            }
            Summary::Completed { interval } => {
                format.print(tasks.iter().filter(|(_, task)| task.is_completed_in_past(&interval)));
            }
        };
        Ok(())
    }
}
