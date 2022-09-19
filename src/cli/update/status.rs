use clap::Subcommand;

use crate::components::{outcome::Outcome, short_string::ShortString, status::Status, tasks::Tasks, TaskId, time::date_specifier::DateSpecifier};

#[derive(Debug, Subcommand)]
pub(crate) enum StatusUpdate {
    /// Task is in progress.
    InProgress,

    /// Task is blocked by another task.
    BlockedByTask { id: TaskId },

    /// Task is blocked for some reason.
    BlockedByOther { reason: ShortString },

    /// Task has been completed.
    Completed {
        #[clap(value_enum)]
        outcome: Outcome,

        #[clap(default_value = "today")]
        date: DateSpecifier,
    },

    /// Task has been discarded for some reason.
    Discarded { reason: ShortString },
}

impl StatusUpdate {
    pub(crate) fn run(self, id: TaskId, tasks: &mut Tasks) -> anyhow::Result<()> {
        let status = match self {
            StatusUpdate::InProgress => Status::in_progress(),
            StatusUpdate::BlockedByTask { id: blocking_id } => {
                Status::blocked_by_task(blocking_id, tasks).expect("could not set status")
            }
            StatusUpdate::BlockedByOther { reason } => Status::blocked_by_other(reason),
            StatusUpdate::Completed { outcome, date } => Status::completed(outcome, date.into()),
            StatusUpdate::Discarded { reason } => Status::discarded(reason),
        };
        tasks.get_task_mut_err(id)?.set_status(status);
        Ok(())
    }
}
