use std::str::FromStr;

use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum SortBy {
    Id,
    Category,
    Priority,
    Deadline
}

impl Default for SortBy {
    fn default() -> Self {
        Self::Priority
    }
}

impl FromStr for SortBy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "id" => Ok(Self::Id),
            "category" => Ok(Self::Category),
            "priority" => Ok(Self::Priority),
            "deadline" => Ok(Self::Deadline),
            _ => Err(anyhow::anyhow!("cannot interpret {s} as a sorting rule"))
        }
    }
}