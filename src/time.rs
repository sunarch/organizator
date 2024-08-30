// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use chrono::{DateTime, Datelike, Days, Local, Month, Months, NaiveDate};

// time intervals for dated display
pub const MONTHS_12: Months = Months::new(12);
pub const DAYS_1: Days = Days::new(1);
pub const DAYS_6: Days = Days::new(6);

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

pub fn subtract_days(date: NaiveDate, count: u16) -> Option<NaiveDate> {
    return date.checked_sub_days(Days::new(count as u64));
}

pub fn today() -> NaiveDate {
    let timestamp: DateTime<Local> = Local::now();
    return NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), timestamp.day())
        .expect("Failed to create NaiveDate from now()");
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
