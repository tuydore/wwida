use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::today;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum TimeInterval {
    Days(usize),
    Weeks(usize),
    Months(usize),
}

impl TimeInterval {
    /// Checks whether a date is in the past throwback interval.
    ///
    /// ```txt
    /// false true
    /// <---- |----------------------->
    /// ------X--------0---------------
    ///       tb date  today
    /// ```
    pub(crate) fn contains(&self, date: &NaiveDate) -> bool {
        let delta = today() - *date;

        // future dates are automatically contained
        if delta.num_days() < 0 {
            return true;
        }

        match self {
            TimeInterval::Days(days) => delta.num_days() as usize <= *days,
            TimeInterval::Weeks(weeks) => delta.num_weeks() as usize <= *weeks,
            TimeInterval::Months(months) => delta.num_days() as usize <= months * 30,
        }
    }
}

impl FromStr for TimeInterval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("days/") {
            str::parse::<usize>(num)
                .map(Self::Days)
                .map_err(|_| anyhow::anyhow!("{num} is not a valid number of days"))
        } else if let Some(num) = s.strip_prefix("weeks/") {
            str::parse::<usize>(num)
                .map(Self::Weeks)
                .map_err(|_| anyhow::anyhow!("{num} is not a valid number of weeks"))
        } else if let Some(num) = s.strip_prefix("months/") {
            str::parse::<usize>(num)
                .map(Self::Months)
                .map_err(|_| anyhow::anyhow!("{num} is not a valid number of months"))
        } else {
            Err(anyhow::anyhow!("cannot interpret {s} as a time unit"))
        }
    }
}
