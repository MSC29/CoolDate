use chrono::{DateTime, Duration, Utc};

use crate::entities::anniversary::{Anniversary};

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

pub fn find_anniversaries_from_date(date: DateTime<Utc>) -> Vec<Anniversary> {
    let mut aniversaries = vec![];
    let mut seconds = create_seconds_anniversaries_from_date(date.clone());
    aniversaries.append(&mut seconds);

    let mut days = create_days_anniversaries_from_date(date.clone());
    aniversaries.append(&mut days);

    let mut weeks = create_weeks_anniversaries_from_date(date.clone());
    aniversaries.append(&mut weeks);

    return aniversaries;
}

pub fn create_seconds_anniversaries_from_date(date: DateTime<Utc>) -> Vec<Anniversary> {
    let mut aniversaries: Vec<Anniversary> = vec![];

    for sec in COOL_SECONDS {
        let duration = Duration::seconds(sec.into());
        let date = date.checked_add_signed(duration);
        let annif = Anniversary{
            count: sec,
            date: date.unwrap(),
            name: String::from(format!("{} seconds", sec)),
            unit: String::from("seconds")
        };
        aniversaries.push(annif);
    }

    return aniversaries;
}

pub fn create_days_anniversaries_from_date(date: DateTime<Utc>) -> Vec<Anniversary> {
    let mut aniversaries: Vec<Anniversary> = vec![];

    for day in COOL_DAYS {
        let duration = Duration::days(day.into());
        let date = date.checked_add_signed(duration);
        let annif = Anniversary{
            count: day,
            date: date.unwrap(),
            name: String::from(format!("{} days", day)),
            unit: String::from("days")
        };
        aniversaries.push(annif);
    }

    return aniversaries;
}

pub fn create_weeks_anniversaries_from_date(date: DateTime<Utc>) -> Vec<Anniversary> {
    let mut aniversaries: Vec<Anniversary> = vec![];

    for week in COOL_WEEKS {
        let duration = Duration::weeks(week.into());
        let date = date.checked_add_signed(duration);
        let annif = Anniversary{
            count: week,
            date: date.unwrap(),
            name: String::from(format!("{} weeks", week)),
            unit: String::from("weeks")
        };
        aniversaries.push(annif);
    }

    return aniversaries;
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};

    use crate::dateUtils;

    #[test]
    fn adding_seconds() {
        //given

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);
        
        //2041-09-10T04:50:45
        let date_expected_1_000_d_000_000_s = Utc.ymd(2041, 9, 10).and_hms(4, 50, 45);

        //when
        let vec = dateUtils::create_seconds_anniversaries_from_date(date_start);
       
        //then
        assert_eq!(vec[0].date, date_expected_1_000_d_000_000_s);
    }

    #[test]
    fn adding_days() {
        //given

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);
        
        //Sunday, 30 October 2011, 03:04:05
        let date_expected_666_d = Utc.ymd(2011, 10, 30).and_hms(3, 4, 5);
        //Friday, 28 September 2012, 03:04:05
        let date_expected_1_000_d = Utc.ymd(2012, 9, 28).and_hms(3, 4, 5);
        //Wednesday, 20 May 2037, 03:04:05
        let date_expected_10_000_d = Utc.ymd(2037, 5, 20).and_hms(3, 4, 5);

        //when
        let vec = dateUtils::create_days_anniversaries_from_date(date_start);
        
        //then
        assert_eq!(vec[0].date, date_expected_666_d);
        assert_eq!(vec[1].date, date_expected_1_000_d);
        assert_eq!(vec[2].date, date_expected_10_000_d);
    }

    #[test]
    fn adding_weeks() {
        //given

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);
        
        //Saturday, 8 October 2022, 03:04:05
        let date_expected_666_w = Utc.ymd(2022, 10, 8).and_hms(3, 4, 5);
        //Saturday, 3 March 2029, 03:04:05
        let date_expected_1_000_w = Utc.ymd(2029, 3, 3).and_hms(3, 4, 5);
        
        //when
        let vec = dateUtils::create_weeks_anniversaries_from_date(date_start);
        
        //then
        assert_eq!(vec[0].date, date_expected_666_w);
        assert_eq!(vec[1].date, date_expected_1_000_w);
    }

    #[test]
    fn finding_anniversaries_from_date() {
        //given
        
        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);
        
        //2041-09-10T04:50:45
        let date_expected_1_000_d_000_000_s = Utc.ymd(2041, 9, 10).and_hms(4, 50, 45);
       
        //Sunday, 30 October 2011, 03:04:05
        let date_expected_666_d = Utc.ymd(2011, 10, 30).and_hms(3, 4, 5);
        //Friday, 28 September 2012, 03:04:05
        let date_expected_1_000_d = Utc.ymd(2012, 9, 28).and_hms(3, 4, 5);
        //Wednesday, 20 May 2037, 03:04:05
        let date_expected_10_000_d = Utc.ymd(2037, 5, 20).and_hms(3, 4, 5);

        //Saturday, 8 October 2022, 03:04:05
        let date_expected_666_w = Utc.ymd(2022, 10, 8).and_hms(3, 4, 5);
        //Saturday, 3 March 2029, 03:04:05
        let date_expected_1_000_w = Utc.ymd(2029, 3, 3).and_hms(3, 4, 5);
        
        //when
        let vec = dateUtils::find_anniversaries_from_date(date_start);
        
        //then
        assert_eq!(vec[0].date, date_expected_1_000_d_000_000_s);

        assert_eq!(vec[1].date, date_expected_666_d);
        assert_eq!(vec[2].date, date_expected_1_000_d);
        assert_eq!(vec[3].date, date_expected_10_000_d);

        assert_eq!(vec[4].date, date_expected_666_w);
        assert_eq!(vec[5].date, date_expected_1_000_w);
    }
}