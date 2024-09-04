// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod parsing;
pub mod timestamp;

use std::cmp::Ordering;
// dependencies
use chrono::{DateTime, Datelike, Days, Local, Month, Months, NaiveDate, NaiveWeek, Weekday};

const MONTHS_12: Months = Months::new(12);

const DAYS_7: Days = Days::new(7);
const DAYS_6: Days = Days::new(6);
const DAYS_1: Days = Days::new(1);

pub fn add_years(date: &NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_months(Months::new((count * 12) as u32));
}

pub fn add_months(date: &NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_months(Months::new(count as u32));
}

pub fn add_weeks(date: &NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_days(Days::new((count * 7) as u64));
}

pub fn add_days(date: &NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_days(Days::new(count as u64));
}

pub fn increment_by_one_week(date: &NaiveDate) -> NaiveDate {
    return date.checked_add_days(DAYS_7).expect("Failed to add days.");
}

pub fn increment_by_one_day(date: &NaiveDate) -> NaiveDate {
    return date.checked_add_days(DAYS_1).expect("Failed to add day.");
}

pub fn week_of_day(date: &NaiveDate) -> NaiveWeek {
    return date.week(Weekday::Mon);
}

pub fn monday_to_sunday(date: &NaiveDate) -> NaiveDate {
    return date.checked_add_days(DAYS_6).expect("Failed to add days.");
}

pub fn adjust_by_buffer_days(date: &NaiveDate, count: i32) -> Option<NaiveDate> {
    return match count.cmp(&0) {
        Ordering::Equal => Some(*date),
        Ordering::Greater => date.checked_sub_days(Days::new(count as u64)),
        Ordering::Less => date.checked_add_days(Days::new(count.unsigned_abs() as u64)),
    };
}

pub fn today() -> NaiveDate {
    let dt_now: DateTime<Local> = Local::now();
    return NaiveDate::from_ymd_opt(dt_now.year(), dt_now.month(), dt_now.day())
        .expect("Failed to create NaiveDate from now()");
}

pub fn first_sunday_after_12_months(date: &NaiveDate) -> NaiveDate {
    let mut target_date: NaiveDate = date
        .checked_add_months(MONTHS_12)
        .expect("Failed to add months");
    const SUNDAY_VALUE: u8 = 7;
    let add_for_sunday: u32 = SUNDAY_VALUE as u32 - target_date.weekday().num_days_from_sunday();
    if add_for_sunday > 0 {
        target_date = target_date
            .checked_add_days(Days::new(add_for_sunday as u64))
            .expect("Failed to add days");
    }
    return target_date;
}

pub fn next_monday(date: &NaiveDate) -> NaiveDate {
    const DAY_COUNT: u8 = 7;
    let add_for_monday: u32 = DAY_COUNT as u32 - date.weekday().num_days_from_monday();
    return if add_for_monday > 0 {
        date.checked_add_days(Days::new(add_for_monday as u64))
            .expect("Failed to add days")
    } else {
        *date
    };
}

pub fn iterate_week(week: &NaiveWeek) -> impl Iterator<Item = NaiveDate> {
    const DAYS_IN_WEEK: usize = 7;
    return week.first_day().iter_days().take(DAYS_IN_WEEK);
}

pub fn month_abbrev(month: u32) -> String {
    let month: Month = Month::try_from(month as u8).expect("Failed to convert month.");
    let month_name: &str = month.name();
    let mut name_abbrev: String = month_name.to_string().drain(0..3).as_str().to_string();
    if month != Month::May {
        name_abbrev.push('.');
    }
    return name_abbrev;
}

pub fn weekday_abbrev(date: &NaiveDate) -> String {
    return date.weekday().to_string();
}

pub fn is_day_in_first_week_of_year(date: &NaiveDate) -> bool {
    return format!("{:?}", date.iso_week()).ends_with("01");
}
