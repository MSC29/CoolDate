use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Anniversary {
    pub name: String,
    pub count: i64,
    pub unit: String,
    pub date: DateTime<Utc>,
}

impl Anniversary {
    pub fn new_seconds(count: i64, date: DateTime<Utc>) -> Anniversary {
        Anniversary {
            count,
            date,
            name: String::from(format!("{} seconds", count)),
            unit: String::from("seconds"),
        }
    }

    pub fn new_days(count: i64, date: DateTime<Utc>) -> Anniversary {
        Anniversary {
            count,
            date,
            name: String::from(format!("{} days", count)),
            unit: String::from("days"),
        }
    }

    pub fn new_weeks(count: i64, date: DateTime<Utc>) -> Anniversary {
        Anniversary {
            count,
            date,
            name: String::from(format!("{} weeks", count)),
            unit: String::from("weeks"),
        }
    }
}
