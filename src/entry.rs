use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Entry {
    date: DateTime<Utc>,
    title: String,
    info: Option<String>,
    is_complete: bool,
}

impl Entry {
    pub fn new(title: String, info: Option<String>, is_complete: bool) -> Self {
        Self {
            date: Utc::now(),
            title,
            info,
            is_complete,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pattern = "%Y-%m-%d %H:%M:%S";
        let date_formatted = self.date.format(pattern).to_string();

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
