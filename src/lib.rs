use chrono::{DateTime, Utc};
use entities::anniversary::Anniversary;

use crate::date_service::DateService;

mod date_service;
pub mod entities;

const FUN_ANNIVERSARIES_COUNT: [i64; 44] = [
    42,
    314,
    12345,
    123456,
    1234567,
    12345678,
    123456789,
    9876543210,
    876543210,
    76543210,
    6543210,
    543210,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    111,
    1111,
    11111,
    111111,
    1111111,
    11111111,
    111111111,
    1111111111,
    11111111111,
    111111111111,
    1111111111111,
    11111111111111,
    333,
    4444,
    55555,
    666,
    666666,
    777,
    7777777,
    88888888,
    999999999,
];

pub fn find_anniversaries_future(date_str: &str) -> Vec<Anniversary> {
    find_anniversaries(date_str, Some(false))
}

pub fn find_anniversaries_past(date_str: &str) -> Vec<Anniversary> {
    find_anniversaries(date_str, Some(true))
}

pub fn find_anniversaries_all(date_str: &str) -> Vec<Anniversary> {
    find_anniversaries(date_str, None)
}

fn find_anniversaries(date_str: &str, is_past: Option<bool>) -> Vec<Anniversary> {
    let now: DateTime<Utc> = Utc::now();
    let date_service = DateService {
        now,
        fun_anniversaries_count: FUN_ANNIVERSARIES_COUNT.to_vec(),
    };

    let date = DateTime::parse_from_rfc3339(date_str);
    match date {
        Ok(d) => date_service.find_anniversaries_from_date(d.with_timezone(&Utc), is_past),
        Err(e) => panic!("error converting date: {}", e),
    }
}
