// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;
// dependencies
use chrono::{DateTime, Datelike, Days, Local, Month, Months, NaiveDate, NaiveWeek, Weekday};

// time intervals for dated display
const MONTHS_12: Months = Months::new(12);
const DAYS_6: Days = Days::new(6);
const DAYS_7: Days = Days::new(7);

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

pub fn increment_by_one_week(date_ref: &NaiveDate) -> NaiveDate {
    return date_ref
        .checked_add_days(DAYS_7)
        .expect("Failed to add days.");
}

pub fn week_of_day(date_ref: &NaiveDate) -> NaiveWeek {
    return date_ref.week(Weekday::Mon);
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

pub fn next_monday(date_ref: &NaiveDate) -> NaiveDate {
    const DAY_COUNT: u8 = 7;
    let add_for_monday: u32 = DAY_COUNT as u32 - date_ref.weekday().num_days_from_monday();
    return if add_for_monday > 0 {
        date_ref
            .checked_add_days(Days::new(add_for_monday as u64))
            .expect("Failed to add days")
    } else {
        *date_ref
    };
}

pub fn iterate_week(week_ref: &NaiveWeek) -> impl Iterator<Item = NaiveDate> {
    const DAYS_IN_WEEK: usize = 7;
    return week_ref.first_day().iter_days().take(DAYS_IN_WEEK);
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

pub fn week_timestamp(date_ref: &NaiveDate) -> String {
    let date_monday: NaiveDate = if date_ref.weekday() == Weekday::Mon {
        *date_ref
    } else {
        let subtract_for_monday: u32 = date_ref.weekday().num_days_from_monday();
        date_ref
            .checked_sub_days(Days::new(subtract_for_monday as u64))
            .expect("Failed to subtract days")
    };
    let date_sunday: NaiveDate = monday_to_sunday(&date_monday);

    let date_range_display: String = if date_monday.month() == date_sunday.month() {
        format!(
            "{} {}-{}.",
            month_abbrev(date_monday.month()),
            date_monday.day(),
            date_sunday.day()
        )
    } else {
        format!(
            "{} {}. - {} {}.",
            month_abbrev(date_monday.month()),
            date_monday.day(),
            month_abbrev(date_sunday.month()),
            date_sunday.day(),
        )
    };

    return format!("#### {:?} ({})", date_monday.iso_week(), date_range_display);
}

pub fn is_day_in_first_week_of_year(date_ref: &NaiveDate) -> bool {
    return format!("{:?}", date_ref.iso_week()).ends_with("01");
}

pub fn current_clock_timestamp() -> String {
    let date: DateTime<Local> = Local::now();
    return format!("{}", date.format("%H:%M:%S"));
}
