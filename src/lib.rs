use chrono::{DateTime, Duration, Utc};
use entities::anniversary::Anniversary;

mod entities;
mod dateUtils;

pub fn find_anniversaries_future(date: DateTime<Utc>) -> Vec<Anniversary> {
    println!("find_anniversaries_future");
    dateUtils::find_anniversaries_from_date(date)
}