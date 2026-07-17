use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Utc};

pub struct Entry {
    date: DateTime<Utc>,
    title: String,
    info: Option<String>,
}

impl Entry {
    pub fn new(title: String, info: Option<String>) -> Self {
        Self {
            date: Utc::now(),
            title,
            info,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pattern = "%Y-%m-%d %H:%M:%S";
        let date_formatted = self.date.format(pattern).to_string();

        match &self.info {
            Some(details) => write!(f, "{} - [{}]: {}", date_formatted, self.title, details),
            None => write!(f, "{} - [{}]", date_formatted, self.title),
        }
    }
}
