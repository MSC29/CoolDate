use chrono::{DateTime, Duration, Utc};

use crate::entities::anniversary::Anniversary;

/// The number of (non-leap) seconds in days.
const SECS_PER_DAY: i64 = 86400;
/// The number of (non-leap) seconds in a week.
const SECS_PER_WEEK: i64 = 604800;

pub struct DateService {
    pub now: DateTime<Utc>,
    pub fun_anniversaries_count: Vec<i64>,
}

impl DateService {
    pub fn find_anniversaries_from_date(
        &self,
        date: DateTime<Utc>,
        is_past: Option<bool>,
    ) -> Vec<Anniversary> {
        let mut anniversaries: Vec<Anniversary> = vec![];
        let mut seconds: Vec<Anniversary> =
            self.create_seconds_anniversaries_from_date(date.clone());
        anniversaries.append(&mut seconds);

        let mut days: Vec<Anniversary> = self.create_days_anniversaries_from_date(date.clone());
        anniversaries.append(&mut days);

        let mut weeks: Vec<Anniversary> = self.create_weeks_anniversaries_from_date(date.clone());
        anniversaries.append(&mut weeks);

        if let Some(past) = is_past {
            anniversaries = self.filter_anniversaries(anniversaries, past);
        }

        return anniversaries;
    }

    pub fn filter_anniversaries(
        &self,
        anniversaries: Vec<Anniversary>,
        is_past: bool,
    ) -> Vec<Anniversary> {
        let filtered: Vec<Anniversary>;
        if is_past == true {
            filtered = anniversaries
                .into_iter()
                .filter(|a| a.date.lt(&self.now))
                .collect();
        } else {
            filtered = anniversaries
                .into_iter()
                .filter(|a| a.date.ge(&self.now))
                .collect();
        }

        filtered
    }

    pub fn create_seconds_anniversaries_from_date(&self, date: DateTime<Utc>) -> Vec<Anniversary> {
        let mut anniversaries: Vec<Anniversary> = vec![];

        for sec in self.fun_anniversaries_count.iter() {
            if let Some(_) = sec.checked_mul(1000) {
                let duration = Duration::seconds(sec.to_owned() as i64);
                let date = date.checked_add_signed(duration);
                if let Some(d) = date {
                    let annif = Anniversary::new_seconds(sec.to_owned(), d);
                    anniversaries.push(annif);
                } else {
                    println!("Overflow calculating seconds from {}", sec);
                }
            } else {
                println!("Too many milliseconds in seconds");
            }
        }

        return anniversaries;
    }

    pub fn create_days_anniversaries_from_date(&self, date: DateTime<Utc>) -> Vec<Anniversary> {
        let mut anniversaries: Vec<Anniversary> = vec![];

        for day in self.fun_anniversaries_count.iter() {
            if let Some(_) = day.checked_mul(SECS_PER_DAY * 1000) {
                let duration = Duration::days(day.to_owned() as i64);
                let date = date.checked_add_signed(duration);
                if let Some(d) = date {
                    let annif = Anniversary::new_days(day.to_owned(), d);
                    anniversaries.push(annif);
                } else {
                    println!("Overflow calculating days from {}", day);
                }
            } else {
                println!("Too many milliseconds in days");
            }
        }

        return anniversaries;
    }

    pub fn create_weeks_anniversaries_from_date(&self, date: DateTime<Utc>) -> Vec<Anniversary> {
        let mut anniversaries: Vec<Anniversary> = vec![];

        for week in self.fun_anniversaries_count.iter() {
            if let Some(_) = week.checked_mul(SECS_PER_WEEK * 1000) {
                let duration = Duration::weeks(week.to_owned() as i64);
                let date = date.checked_add_signed(duration);
                if let Some(d) = date {
                    let annif = Anniversary::new_weeks(week.to_owned(), d);
                    anniversaries.push(annif);
                } else {
                    println!("Overflow calculating weeks from {}", week);
                }
            } else {
                println!("Too many milliseconds in weeks");
            }
        }

        return anniversaries;
    }
}

#[cfg(test)]
mod tests {
    use crate::date_service::DateService;
    use crate::entities::anniversary::Anniversary;
    use chrono::{DateTime, TimeZone, Utc};

    const TEST_HIGH: [i64; 1] = [1_000_000_000];
    const TEST_MEDIUM: [i64; 3] = [666, 1_000, 10_000];
    const TEST_LOW: [i64; 2] = [666, 1_000];

    const TEST_ALL: [i64; 4] = [666, 1_000, 10_000, 1_000_000_000];

    #[test]
    fn adding_seconds() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService {
            now,
            fun_anniversaries_count: TEST_HIGH.to_vec(),
        };

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);

        //2041-09-10T04:50:45
        let date_expected_1_000_d_000_000_s = Utc.ymd(2041, 9, 10).and_hms(4, 50, 45);

        //when
        let vec = date_service.create_seconds_anniversaries_from_date(date_start);

        //then
        assert_eq!(vec[0].date, date_expected_1_000_d_000_000_s);
    }

    #[test]
    fn adding_days() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService {
            now,
            fun_anniversaries_count: TEST_MEDIUM.to_vec(),
        };

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);

        //Sunday, 30 October 2011, 03:04:05
        let date_expected_666_d = Utc.ymd(2011, 10, 30).and_hms(3, 4, 5);
        //Friday, 28 September 2012, 03:04:05
        let date_expected_1_000_d = Utc.ymd(2012, 9, 28).and_hms(3, 4, 5);
        //Wednesday, 20 May 2037, 03:04:05
        let date_expected_10_000_d = Utc.ymd(2037, 5, 20).and_hms(3, 4, 5);

        //when
        let vec = date_service.create_days_anniversaries_from_date(date_start);

        //then
        assert_eq!(vec[0].date, date_expected_666_d);
        assert_eq!(vec[1].date, date_expected_1_000_d);
        assert_eq!(vec[2].date, date_expected_10_000_d);
    }

    #[test]
    fn adding_weeks() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService {
            now,
            fun_anniversaries_count: TEST_LOW.to_vec(),
        };

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);

        //Saturday, 8 October 2022, 03:04:05
        let date_expected_666_w = Utc.ymd(2022, 10, 8).and_hms(3, 4, 5);
        //Saturday, 3 March 2029, 03:04:05
        let date_expected_1_000_w = Utc.ymd(2029, 3, 3).and_hms(3, 4, 5);

        //when
        let vec = date_service.create_weeks_anniversaries_from_date(date_start);

        //then
        assert_eq!(vec[0].date, date_expected_666_w);
        assert_eq!(vec[1].date, date_expected_1_000_w);
    }

    #[test]
    fn filter_anniversaries_in_future() {
        //given
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);
        let date_service = DateService {
            now: date_now,
            fun_anniversaries_count: TEST_ALL.to_vec(),
        };
        let mut anniversaries: Vec<Anniversary> = vec![];

        //2041-09-10T04:50:45
        let date_expected_1_000_d_000_000_s = Utc.ymd(2041, 9, 10).and_hms(4, 50, 45);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_1_000_d_000_000_s));

        //Sunday, 30 October 2011, 03:04:05
        let date_expected_666_d = Utc.ymd(2011, 10, 30).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_666_d));

        //Friday, 28 September 2012, 03:04:05
        let date_expected_1_000_d = Utc.ymd(2012, 9, 28).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_1_000_d));

        //Wednesday, 20 May 2037, 03:04:05
        let date_expected_10_000_d = Utc.ymd(2037, 5, 20).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_10_000_d));

        //Saturday, 8 October 2022, 03:04:05
        let date_expected_666_w = Utc.ymd(2022, 10, 8).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_666_w));

        //Saturday, 3 March 2029, 03:04:05
        let date_expected_1_000_w = Utc.ymd(2029, 3, 3).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_1_000_w));

        //when
        let vec = date_service.filter_anniversaries(anniversaries, false);

        //then
        assert_eq!(vec[0].date, date_expected_1_000_d_000_000_s);
        assert_eq!(vec[1].date, date_expected_10_000_d);
        assert_eq!(vec[2].date, date_expected_666_w);
        assert_eq!(vec[3].date, date_expected_1_000_w);
    }

    #[test]
    fn filter_anniversaries_in_past() {
        //given
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);
        let date_service = DateService {
            now: date_now,
            fun_anniversaries_count: TEST_ALL.to_vec(),
        };
        let mut anniversaries: Vec<Anniversary> = vec![];

        //2041-09-10T04:50:45
        let date_expected_1_000_d_000_000_s = Utc.ymd(2041, 9, 10).and_hms(4, 50, 45);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_1_000_d_000_000_s));

        //Sunday, 30 October 2011, 03:04:05
        let date_expected_666_d = Utc.ymd(2011, 10, 30).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_666_d));

        //Friday, 28 September 2012, 03:04:05
        let date_expected_1_000_d = Utc.ymd(2012, 9, 28).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_1_000_d));

        //Wednesday, 20 May 2037, 03:04:05
        let date_expected_10_000_d = Utc.ymd(2037, 5, 20).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_10_000_d));

        //Saturday, 8 October 2022, 03:04:05
        let date_expected_666_w = Utc.ymd(2022, 10, 8).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_666_w));

        //Saturday, 3 March 2029, 03:04:05
        let date_expected_1_000_w = Utc.ymd(2029, 3, 3).and_hms(3, 4, 5);
        anniversaries.push(Anniversary::new_seconds(1, date_expected_1_000_w));

        //when
        let vec = date_service.filter_anniversaries(anniversaries, true);

        //then
        assert_eq!(vec[0].date, date_expected_666_d);
        assert_eq!(vec[1].date, date_expected_1_000_d);
    }

    #[test]
    fn finding_anniversaries_from_date_all() {
        //given
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);
        let date_service = DateService {
            now: date_now,
            fun_anniversaries_count: TEST_ALL.to_vec(),
        };

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);

        //when
        let vec = date_service.find_anniversaries_from_date(date_start, None);

        //then
        assert_eq!(vec.len(), 10);
    }

    #[test]
    fn finding_anniversaries_from_date_past() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService {
            now,
            fun_anniversaries_count: TEST_ALL.to_vec(),
        };
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);

        //when
        let vec = date_service.find_anniversaries_from_date(date_start, Some(true));

        //then
        vec.into_iter()
            .for_each(|a| assert_eq!(a.date.lt(&date_now), true));
    }

    #[test]
    fn finding_anniversaries_from_date_future() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService {
            now,
            fun_anniversaries_count: TEST_ALL.to_vec(),
        };
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);

        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);

        //when
        let vec = date_service.find_anniversaries_from_date(date_start, Some(false));

        //then
        vec.into_iter()
            .for_each(|a| assert_eq!(a.date.gt(&date_now), true));
    }
}
