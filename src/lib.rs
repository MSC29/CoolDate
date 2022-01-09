use chrono::{DateTime, Duration, Utc};
use entities::anniversary::Anniversary;
mod entities;

pub fn find_anniversaries_past(date: DateTime<Utc>) -> Vec<Anniversary> {
    println!("find_anniversaries_past");
    vec![]
}

pub fn find_anniversaries_future() {
    println!("find_anniversaries_future");
}

pub fn find_anniversaries_from_count() {
    println!("find_anniversaries_future");
}