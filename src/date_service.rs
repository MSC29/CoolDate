use chrono::{DateTime, Duration, Utc};

use crate::entities::anniversary::Anniversary;

pub struct DateService{
    pub now: DateTime<Utc>,
    pub cool_seconds: [u32; 1],
    pub cool_days: [u32; 3],
    pub cool_weeks: [u32; 2],
}

impl DateService{
    pub fn find_anniversaries_from_date(&self, date: DateTime<Utc>, is_past: Option<bool>) -> Vec<Anniversary> {
        let mut aniversaries: Vec<Anniversary> = vec![];
        let mut seconds: Vec<Anniversary> = self.create_seconds_anniversaries_from_date(date.clone());
        aniversaries.append(&mut seconds);

        let mut days: Vec<Anniversary> = self.create_days_anniversaries_from_date(date.clone());
        aniversaries.append(&mut days);

        let mut weeks: Vec<Anniversary> = self.create_weeks_anniversaries_from_date(date.clone());
        aniversaries.append(&mut weeks);

        if let Some(past) = is_past {
            aniversaries = self.filter_anniversaries(aniversaries, past);
        }

        return aniversaries;
    }

    pub fn filter_anniversaries(&self, aniversaries: Vec<Anniversary>, is_past: bool) -> Vec<Anniversary>{
        let filtered: Vec<Anniversary>;
        if is_past == true{
            filtered = aniversaries.into_iter().filter(|a| a.date.lt(&self.now)).collect();
        }
        else{
            filtered = aniversaries.into_iter().filter(|a| a.date.ge(&self.now)).collect();
        }

        filtered
    }

    pub fn create_seconds_anniversaries_from_date(&self, date: DateTime<Utc>) -> Vec<Anniversary> {
        let mut aniversaries: Vec<Anniversary> = vec![];

        for sec in self.cool_seconds {
            let duration = Duration::seconds(sec.into());
            let date = date.checked_add_signed(duration);
            let annif = Anniversary::new_seconds(sec, date.unwrap());
            aniversaries.push(annif);
        }

        return aniversaries;
    }

    pub fn create_days_anniversaries_from_date(&self, date: DateTime<Utc>) -> Vec<Anniversary> {
        let mut aniversaries: Vec<Anniversary> = vec![];

        for day in self.cool_days {
            let duration = Duration::days(day.into());
            let date = date.checked_add_signed(duration);
            let annif = Anniversary::new_days(day, date.unwrap());
            aniversaries.push(annif);
        }

        return aniversaries;
    }

    pub fn create_weeks_anniversaries_from_date(&self, date: DateTime<Utc>) -> Vec<Anniversary> {
        let mut aniversaries: Vec<Anniversary> = vec![];

        for week in self.cool_weeks {
            let duration = Duration::weeks(week.into());
            let date = date.checked_add_signed(duration);
            let annif = Anniversary::new_weeks(week, date.unwrap());
            aniversaries.push(annif);
        }

        return aniversaries;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone, DateTime};
    use crate::entities::anniversary::Anniversary;
    use crate::date_service::DateService;

    const TEST_COOL_SECONDS: [u32; 1] = [
        1_000_000_000
    ];

    const TEST_COOL_DAYS: [u32; 3] = [
        666,
        1_000,
        10_000
    ];

    const TEST_COOL_WEEKS: [u32; 2] = [
        666,
        1_000
    ];

    #[test]
    fn adding_seconds() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService{
            now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
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
        let date_service = DateService{
            now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
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
        let date_service = DateService{
            now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
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
        let date_service = DateService{
            now: date_now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
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
        let date_service = DateService{
            now: date_now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
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
        let date_service = DateService{
            now: date_now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
        };
        
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
        let vec = date_service.find_anniversaries_from_date(date_start, None);
        
        //then
        assert_eq!(vec[0].date, date_expected_1_000_d_000_000_s);

        assert_eq!(vec[1].date, date_expected_666_d);
        assert_eq!(vec[2].date, date_expected_1_000_d);
        assert_eq!(vec[3].date, date_expected_10_000_d);

        assert_eq!(vec[4].date, date_expected_666_w);
        assert_eq!(vec[5].date, date_expected_1_000_w);
    }

    #[test]
    fn finding_anniversaries_from_date_past() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService{
            now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
        };
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);
        
        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);
        
        //when
        let vec = date_service.find_anniversaries_from_date(date_start, Some(true));
        
        //then
        vec.into_iter().for_each(|a| assert_eq!(a.date.lt(&date_now), true));
    }


    #[test]
    fn finding_anniversaries_from_date_future() {
        //given
        let now: DateTime<Utc> = Utc::now();
        let date_service = DateService{
            now, 
            cool_seconds: TEST_COOL_SECONDS, 
            cool_days: TEST_COOL_DAYS, 
            cool_weeks: TEST_COOL_WEEKS, 
        };
        let date_now = Utc.ymd(2020, 1, 2).and_hms(3, 4, 5);
        
        //2010-01-02T03:04:05
        let date_start = Utc.ymd(2010, 1, 2).and_hms(3, 4, 5);
        
        //when
        let vec = date_service.find_anniversaries_from_date(date_start, Some(false));
        
        //then
        vec.into_iter().for_each(|a| assert_eq!(a.date.gt(&date_now), true));
    }

}