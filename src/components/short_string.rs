use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

pub(crate) const SHORT_STRING_THRESHOLD: usize = 50;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ShortString(String);

impl FromStr for ShortString {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(anyhow::anyhow!("short string cannot be empty"))
        } else if s.chars().count() <= SHORT_STRING_THRESHOLD {
            Ok(Self(s.to_string()))
        } else {
            Err(anyhow::anyhow!(
                "string must have fewer than {SHORT_STRING_THRESHOLD} characters"
            ))
        }
    }
}

impl Display for ShortString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
