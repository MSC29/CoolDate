use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Anniversary{
    pub name: String,
    pub count: u32,
    pub unit: String,
    pub date:  DateTime<Utc>
}