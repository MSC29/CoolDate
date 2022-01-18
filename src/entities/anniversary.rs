use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Anniversary {
    pub name: String,
    pub count: u32,
    pub unit: String,
    pub date: DateTime<Utc>,
}

impl Anniversary {
    pub fn new_seconds(count: u32, date: DateTime<Utc>) -> Anniversary {
        Anniversary {
            count,
            date,
            name: String::from(format!("{} seconds", count)),
            unit: String::from("seconds"),
        }
    }

    pub fn new_days(count: u32, date: DateTime<Utc>) -> Anniversary {
        Anniversary {
            count,
            date,
            name: String::from(format!("{} days", count)),
            unit: String::from("days"),
        }
    }

    pub fn new_weeks(count: u32, date: DateTime<Utc>) -> Anniversary {
        Anniversary {
            count,
            date,
            name: String::from(format!("{} weeks", count)),
            unit: String::from("weeks"),
        }
    }
}
