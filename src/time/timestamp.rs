// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use chrono::{DateTime, Datelike, Days, Local, NaiveDate, Weekday};
// internal
use crate::time;

pub fn week(date: &NaiveDate, align_to_middle: bool) -> String {
    let date_monday: NaiveDate = if date.weekday() == Weekday::Mon {
        *date
    } else {
        let subtract_for_monday: u32 = date.weekday().num_days_from_monday();
        date.checked_sub_days(Days::new(subtract_for_monday as u64))
            .expect("Failed to subtract days")
    };
    let date_sunday: NaiveDate = time::monday_to_sunday(&date_monday);

    let date_range_display: String = if date_monday.month() == date_sunday.month() {
        format!(
            "{} {}-{}.",
            time::month_abbrev(date_monday.month()),
            date_monday.day(),
            date_sunday.day()
        )
    } else {
        format!(
            "{} {}. - {} {}.",
            time::month_abbrev(date_monday.month()),
            date_monday.day(),
            time::month_abbrev(date_sunday.month()),
            date_sunday.day(),
        )
    };

    return match align_to_middle {
        false => format!("{:?} ({})", date_monday.iso_week(), date_range_display),
        true => format!(
            "{: >13} {: <20}",
            format!("{:?}", date_monday.iso_week()),
            format!("({})", date_range_display)
        ),
    };
}

pub fn day(date: &NaiveDate) -> String {
    return format!(
        "{}-{:0>2}-{:0>2} ({})",
        date.year(),
        date.month(),
        date.day(),
        time::weekday_abbrev(date)
    );
}

pub fn day_short(date: &NaiveDate) -> String {
    return format!(
        "{} {}. ({})",
        time::month_abbrev(date.month()),
        date.day(),
        time::weekday_abbrev(date)
    );
}

pub fn current_clock() -> String {
    let date: DateTime<Local> = Local::now();
    return format!("{}", date.format("%H:%M:%S"));
}
