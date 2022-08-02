use clap::Subcommand;

use crate::components::{
    category::Category, short_string::ShortString, tasks::Tasks, time::date_specifier::DateSpecifier, TaskId, priority::Priority, tag::tags_from_comma_separated_string,
};

use self::status::StatusUpdate;

pub(crate) mod status;

#[derive(Debug, Subcommand)]
pub(crate) enum Update {
    /// Updates the tasks's short description.
    Short { short: ShortString },

    /// Updates the tasks's long description.
    Long { long: String },

    /// Removes the task's long description.
    DiscardLong,

    /// Updates the tasks's category.
    Category {
        #[clap(value_enum)]
        category: Category,
    },

    /// Updates the tasks's status.
    Status {
        #[clap(subcommand)]
        status: StatusUpdate,
    },

    /// Updates the task's deadline.
    Deadline { deadline: DateSpecifier },

    /// Removes the task's long description.
    DiscardDeadline,

    /// Update the task's priority.
    Priority { priority: Priority },

    /// Add one or more tags.
    AddTags { tags: String },

    /// Remove one or more tags.
    RemoveTags { tags: String },
}

impl Update {
    pub(crate) fn run(self, id: TaskId, tasks: &mut Tasks) -> anyhow::Result<()> {
        match self {
            Update::Short { short } => tasks.get_task_mut_err(id)?.set_short(short),
            Update::Long { long } => tasks.get_task_mut_err(id)?.set_long(long),
            Update::DiscardLong => tasks.get_task_mut_err(id)?.unset_long(),
            Update::Category { category } => tasks.get_task_mut_err(id)?.set_category(category),
            Update::Status { status } => status.run(id, tasks)?,
            Update::Deadline { deadline } => tasks.get_task_mut_err(id)?.set_deadline(deadline),
            Update::DiscardDeadline => tasks.get_task_mut_err(id)?.unset_deadline(),
            Update::Priority { priority } => tasks.get_task_mut_err(id)?.set_priority(priority),
            Update::AddTags { tags } => {
                let tags = tags_from_comma_separated_string(tags)?;
                tasks.get_task_mut_err(id)?.add_tags(tags);
            },
            Update::RemoveTags { tags } => {
                let tags = tags_from_comma_separated_string(tags)?;
                tasks.get_task_mut_err(id)?.remove_tags(&tags);
            },
        };
        Ok(())
    }
}
