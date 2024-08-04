// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{
    Datelike, DateTime, Local,
    Days, Months,
    Weekday,
};


pub fn print_list() {
    let dt_now: DateTime<Local> = Local::now();
    print_year_heading(dt_now.year());
    let dt_last: DateTime<Local> = dt_now.checked_add_months(Months::new(12)).expect("Failed adding months");
    let mut dt_next: DateTime<Local> = dt_now;
    loop {
        dt_next = dt_next.checked_add_days(Days::new(1)).expect("Failed adding day");

        if dt_next.weekday() == Weekday::Mon {
            if format!("{:?}", dt_next.iso_week()).ends_with("01") {
                print_year_heading(dt_next.year());
            }
            print_week_heading(&dt_next)
        }

        if dt_next.eq(&dt_last) {
            break;
        }
    }
}

fn print_year_heading(year: i32) {
    println!("---");
    println!();
    println!("## {year}");
}

fn print_week_heading(dt_next: &DateTime<Local>) {
    let dt_sunday: DateTime<Local> = dt_next.checked_add_days(Days::new(6)).expect("Failed adding days");
    let mut date_range_display: String = format!("{}.{}", dt_next.month(), dt_next.day());

    if dt_next.month() == dt_sunday.month() {
        date_range_display = format!("{}-{}.",date_range_display, dt_sunday.day());
    } else {
        date_range_display = format!("{}.-{}.{}.", date_range_display, dt_sunday.month(), dt_sunday.day());
    }

    println!();
    println!("#### {:?} ({})", dt_next.iso_week(), date_range_display);
    println!();
}
