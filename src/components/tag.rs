use std::{str::FromStr, fmt::Display};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Tag(String);

impl FromStr for Tag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_alphanumeric() || c == '-') {
            Ok(Self(s.to_string()))
        } else {
            Err(anyhow::anyhow!("{s} must be alphanumeric or '-'"))
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}