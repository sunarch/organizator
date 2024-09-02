// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;
// dependencies
use chrono::{DateTime, Datelike, Days, Local, Month, Months, NaiveDate, Weekday};

// time intervals for dated display
const MONTHS_12: Months = Months::new(12);
const DAYS_1: Days = Days::new(1);
const DAYS_6: Days = Days::new(6);

// time intervals for task frequency

pub fn add_years(date: NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_months(Months::new((count * 12) as u32));
}

pub fn add_months(date: NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_months(Months::new(count as u32));
}

pub fn add_weeks(date: NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_days(Days::new((count * 7) as u64));
}

pub fn add_days(date: NaiveDate, count: u8) -> Option<NaiveDate> {
    return date.checked_add_days(Days::new(count as u64));
}

pub fn increment_day(date_ref: &NaiveDate) -> NaiveDate {
    return date_ref
        .checked_add_days(DAYS_1)
        .expect("Failed to add day.");
}

pub fn monday_to_sunday(date_ref: &NaiveDate) -> NaiveDate {
    return date_ref
        .checked_add_days(DAYS_6)
        .expect("Failed to add days.");
}

pub fn adjust_by_buffer_days(date: NaiveDate, count: i32) -> Option<NaiveDate> {
    return match count.cmp(&0) {
        Ordering::Equal => Some(date),
        Ordering::Greater => date.checked_sub_days(Days::new(count as u64)),
        Ordering::Less => date.checked_add_days(Days::new(count.unsigned_abs() as u64)),
    };
}

pub fn today() -> NaiveDate {
    let timestamp: DateTime<Local> = Local::now();
    return NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), timestamp.day())
        .expect("Failed to create NaiveDate from now()");
}

pub fn first_sunday_after_12_months(today: NaiveDate) -> NaiveDate {
    let mut target_date: NaiveDate = today
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

pub fn month_abbrev(month: u32) -> String {
    let month: Month = Month::try_from(month as u8).expect("Failed to convert month.");
    let month_name: &str = month.name();
    let mut name_abbrev: String = month_name.to_string().drain(0..3).as_str().to_string();
    if month != Month::May {
        name_abbrev.push('.');
    }
    return name_abbrev;
}

pub fn weekday_abbrev(date_ref: &NaiveDate) -> String {
    return date_ref.weekday().to_string();
}

pub fn day_timestamp(date_ref: &NaiveDate) -> String {
    return format!(
        "{}-{:0>2}-{:0>2} ({})",
        date_ref.year(),
        date_ref.month(),
        date_ref.day(),
        weekday_abbrev(date_ref)
    );
}

pub fn is_monday(date_ref: &NaiveDate) -> bool {
    return date_ref.weekday() == Weekday::Mon;
}

pub fn is_day_in_first_week_of_year(date_ref: &NaiveDate) -> bool {
    return format!("{:?}", date_ref.iso_week()).ends_with("01");
}
