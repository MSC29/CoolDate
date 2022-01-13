use chrono::{DateTime, Utc};
use entities::anniversary::Anniversary;

use crate::date_service::DateService;

mod entities;
mod date_service;

const COOL_SECONDS: [u32; 1] = [
    1_000_000_000
];

const COOL_DAYS: [u32; 3] = [
    666,
    1_000,
    10_000
];

const COOL_WEEKS: [u32; 2] = [
    666,
    1_000
];

pub fn find_anniversaries_future(date_str: &str) -> Vec<Anniversary> {
    println!("find_anniversaries_future");
    let now: DateTime<Utc> = Utc::now();
    let date_service = DateService{
        now, 
        cool_seconds: COOL_SECONDS, 
        cool_days: COOL_DAYS, 
        cool_weeks: COOL_WEEKS, 
    };

    let date = DateTime::parse_from_rfc3339(date_str);
    match date {
        Ok(d) => date_service.find_anniversaries_from_date(d.with_timezone(&Utc), Some(false)),
        Err(e) => panic!("error converting date: {}", e)
    }
}