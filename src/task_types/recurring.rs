// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::{Datelike, NaiveDate};
use serde;
// internal
use crate::task_types::_task_types;
use crate::task_types::_task_types::Frequency;
use crate::tasks::Task;
use crate::time;

pub(crate) const DIR_NAME: &str = "recurring";

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    title: String,
    note: String,
    description: String,

    frequency: Frequency,
    snap_to: String,
    last: String,
}

pub(crate) fn parse(file_path: &Path) -> Option<(NaiveDate, Task)> {
    let data: Data = match _task_types::load(file_path) {
        None => {
            return None;
        }
        Some(data) => data,
    };

    let last_date = match NaiveDate::parse_from_str(data.last.as_str(), "%Y-%m-%d") {
        Err(_) => {
            println!(
                "Failed to convert last date in recurring task: '{}' ({})",
                data.last, data.title
            );
            return None;
        }
        Ok(date) => date,
    };

    if data.frequency.number < 1 {
        println!(
            "Invalid frequency number: '{}' ({})",
            data.frequency.number, data.title
        );
        return None;
    }

    let task_date_option: Option<NaiveDate> = match data.frequency.name.as_str() {
        "year" => time::add_years(last_date, data.frequency.number),
        "month" => time::add_months(last_date, data.frequency.number),
        "week" => time::add_weeks(last_date, data.frequency.number),
        "day" => time::add_days(last_date, data.frequency.number),
        _ => None,
    };

    let planned_date: NaiveDate = match task_date_option {
        None => {
            println!("Unable to parse task frequency ({})", data.title);
            return None;
        }
        Some(date) => date,
    };

    let mut task_date: NaiveDate = planned_date;

    let today: NaiveDate = time::today();

    match data.snap_to.as_str() {
        "none" => {}
        "today" => {
            if planned_date < today {
                task_date = today;
            }
        }
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun" => {
            task_date = today;
            while !task_date.weekday().to_string().eq(&data.snap_to) {
                println!(
                    "'task_date_ref {}' - snap_to '{}'",
                    task_date.weekday().to_string(),
                    &data.snap_to
                );
                task_date = time::add_days(task_date, 1).expect("Failed to add day.");
            }
        }
        _ => {
            println!(
                "Unable to parse task snap_to: '{}' ({})",
                data.snap_to, data.title
            );
            return None;
        }
    };

    return Some((
        task_date,
        Task {
            frequency: format!("{}", data.frequency),
            title: data.title,
            note: data.note,
        },
    ));
}
