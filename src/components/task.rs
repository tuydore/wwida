use std::str::FromStr;

use super::{category::Category, short_string::ShortString, status::Status, time::date_specifier::DateSpecifier};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Task {
    pub(crate) short: ShortString,
    pub(crate) long: Option<String>,
    pub(crate) statuses: Vec<Status>,
    pub(crate) category: Category,
    pub(crate) deadline: Option<NaiveDate>,
}

impl Task {
    pub(crate) fn new(
        short: ShortString,
        category: Option<Category>,
        long: Option<String>,
        deadline: Option<DateSpecifier>,
    ) -> Result<Self> {
        Ok(Self {
            short,
            long,
            statuses: vec![Status::default()],
            category: category.unwrap_or_default(),
            deadline: deadline.map(NaiveDate::from),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_strings(
        short: &str,
        category: Option<&str>,
        long: Option<&str>,
        deadline: Option<&str>,
    ) -> Result<Self> {
        let short = ShortString::from_str(short).with_context(|| "error creating new task")?;

        Self::new(
            short,
            category.map(Category::from_str).transpose()?,
            long.map(|s| s.to_string()),
            deadline.map(DateSpecifier::from_str).transpose()?,
        )
    }

    pub(crate) fn set_short(&mut self, short: ShortString) {
        self.short = short;
    }

    pub(crate) fn set_long<S: Into<String>>(&mut self, long: S) {
        self.long = Some(long.into());
    }

    pub(crate) fn unset_long(&mut self) {
        self.long = None;
    }

    pub(crate) fn set_status(&mut self, status: Status) {
        self.statuses.push(status);
    }

    pub(crate) fn set_category(&mut self, category: Category) {
        self.category = category;
    }

    pub(crate) fn last_status(&self) -> &Status {
        self.statuses.last().expect("no last status")
    }
}
