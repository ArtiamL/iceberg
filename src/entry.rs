use chrono::{DateTime, Utc};
use core::result::Result;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use crate::CliError::{self, ValidationError};

// #[derive(Debug, Serialize, Deserialize)]
// enum CompletionState {
//     Complete,
//     Incomplete,
// }

#[derive(
    Debug,
    Eq,
    PartialEq,
    // Serde
    Serialize,
    Deserialize,
)]
pub struct Entry {
    pub id: u32,
    timestamp: DateTime<Utc>,
    pub title: String,
    info: Option<String>,
    // is_complete: CompletionState,
    is_complete: bool,
}

impl Entry {
    pub fn new(id: u32, title: String, info: Option<String>) -> Result<Self, CliError> {
        if title.trim().is_empty() {
            return Err(ValidationError("Title is empty!".into()));
        };

        if let Some(info_str) = &info {
            if info_str.trim().is_empty() {
                return Err(ValidationError(
                    "Info description cannot contain only blank whitespace!".into(),
                ));
            }
        }

        Ok(Entry {
            id,
            timestamp: Utc::now(),
            title,
            info,
            is_complete: false,
        })
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pattern = "%Y-%m-%d %H:%M:%S";
        let date_formatted = self.timestamp.format(pattern).to_string();

        let is_complete = if self.is_complete { "🗸" } else { "✗" };

        match &self.info {
            Some(details) => write!(
                f,
                "[{}]    {} - [{}]:\n\t{}",
                is_complete, date_formatted, self.title, details
            ),
            None => write!(
                f,
                "[{}]    {} - [{}]",
                is_complete, date_formatted, self.title
            ),
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
