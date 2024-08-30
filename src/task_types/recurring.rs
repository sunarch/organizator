// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::NaiveDate;
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
    last: String,

    #[serde(default = "_task_types::default_false")]
    snap_to_today: bool,
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

    let mut task_date: &NaiveDate = &planned_date;

    let today: NaiveDate = time::today();

    if data.snap_to_today && planned_date < today {
        task_date = &today;
    }

    return Some((
        *task_date,
        Task {
            frequency: format!("{}", data.frequency),
            title: data.title,
            note: data.note,
        },
    ));
}
