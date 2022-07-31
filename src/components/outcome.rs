use std::fmt::Display;

use clap::clap_derive::ArgEnum;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, ArgEnum)]
pub(crate) enum Outcome {
    Success,
    Failure,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outcome::Success => write!(f, "✓"),
            Outcome::Failure => write!(f, "x"),
        }
    }
}
