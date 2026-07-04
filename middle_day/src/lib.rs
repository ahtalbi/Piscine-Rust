use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let date = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;
    
    if date.leap_year() {
        return None;
    }

    Some(NaiveDate::from_yo_opt(year as i32, 183)?.weekday())
}