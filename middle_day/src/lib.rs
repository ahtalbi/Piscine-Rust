use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        return None;
    }

    NaiveDate::from_yo_opt(year as i32, 183).map(|d| d.weekday())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", middle_day(1022));
    }
}
