use std::{fmt::Display, str::FromStr};

use clap::clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub(crate) enum Category {
    Task,
    Feature,
    Bug,
    Maintenance,
    Documentation,
    Other,
    Message,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Feature => write!(f, "FEATURE"),
            Category::Bug => write!(f, "BUG"),
            Category::Maintenance => write!(f, "MAINTENANCE"),
            Category::Documentation => write!(f, "DOCUMENTATION"),
            Category::Other => write!(f, "OTHER"),
            Category::Task => write!(f, "TASK"),
            Category::Message => write!(f, "MESSAGE"),
        }
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "task" => Ok(Self::Task),
            "feature" => Ok(Self::Feature),
            "bug" => Ok(Self::Bug),
            "maintenance" => Ok(Self::Maintenance),
            "documentation" => Ok(Self::Documentation),
            "other" => Ok(Self::Other),
            "message" => Ok(Self::Message),
            s => Err(anyhow::anyhow!("{s} is not a valid task category")),
        }
    }
}

impl Default for Category {
    fn default() -> Self {
        Self::Task
    }
}
