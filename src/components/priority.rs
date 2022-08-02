use std::str::FromStr;

use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Priority {
    VeryLow = 1,
    Low = 2,
    Normal = 3,
    High = 4,
    VeryHigh = 5,
}

impl Priority {
    pub(crate) fn as_symbol(&self) -> &str {
        match self {
            Priority::VeryLow => "★",
            Priority::Low => "★★",
            Priority::Normal => "★★★",
            Priority::High => "★★★★",
            Priority::VeryHigh => "★★★★★",
        }
    }
}

impl FromStr for Priority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "very-low" | "1" => Ok(Self::VeryLow),
            "low" | "2" => Ok(Self::Low),
            "normal" | "3" => Ok(Self::Normal),
            "high" | "4" => Ok(Self::High),
            "very-high" | "5" => Ok(Self::VeryHigh),
            _ => Err(anyhow::anyhow!("cannot interpret {s} as a priority"))
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}