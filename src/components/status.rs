use std::fmt::Display;

use super::{
    outcome::Outcome,
    short_string::ShortString,
    tasks::Tasks,
    time::{today, DATE_FORMAT},
    TaskId,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum Status {
    NotStarted,
    InProgress { date: NaiveDate },
    BlockedByTask { date: NaiveDate, id: TaskId },
    BlockedByOther { date: NaiveDate, reason: ShortString },
    Completed { date: NaiveDate, outcome: Outcome },
    Discarded { date: NaiveDate, reason: ShortString },
}

impl Default for Status {
    fn default() -> Self {
        Self::NotStarted
    }
}

impl Status {
    pub(crate) fn in_progress() -> Self {
        Self::InProgress { date: today() }
    }

    pub(crate) fn blocked_by_task(id: TaskId, tasks: &Tasks) -> anyhow::Result<Self> {
        if !tasks.has_id(id) {
            Err(anyhow::anyhow!("blocking ID {id} does not exist"))
        } else {
            Ok(Self::BlockedByTask { date: today(), id })
        }
    }

    pub(crate) fn blocked_by_other(reason: ShortString) -> Self {
        Self::BlockedByOther { date: today(), reason }
    }

    pub(crate) fn completed(outcome: Outcome) -> Self {
        Self::Completed { date: today(), outcome }
    }

    pub(crate) fn discarded(reason: ShortString) -> Self {
        Self::Discarded { date: today(), reason }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::NotStarted => write!(f, "NOT STARTED"),
            Status::InProgress { date } => write!(f, "{} :: IN PROGRESS", date.format(DATE_FORMAT)),
            Status::BlockedByTask { date, id } => write!(f, "{} :: BLOCKED ID={id}", date.format(DATE_FORMAT)),
            Status::BlockedByOther { date, reason } => write!(f, "{} :: BLOCKED {reason}", date.format(DATE_FORMAT)),
            Status::Completed { date, outcome } => write!(f, "{} :: COMPLETED {outcome}", date.format(DATE_FORMAT)),
            Status::Discarded { date, reason } => write!(f, "{} :: DISCARDED {reason}", date.format(DATE_FORMAT)),
        }?;
        Ok(())
    }
}
