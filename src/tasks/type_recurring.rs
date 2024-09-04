// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde;
// internal
use crate::logging;
use crate::tasks::task::contents::TaskContents;
use crate::tasks::task::meta::TaskMeta;
use crate::tasks::task::Task;
use crate::tasks::task_data::TaskAddable;
use crate::tasks::types;
use crate::tasks::types::Subtask;
use crate::time;

pub(crate) const DIR_NAME: &str = "recurring";

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Frequency {
    pub(crate) number: u8,
    pub(crate) name: String,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return if self.number == 1 {
            write!(f, "{}ly", self.name)
        } else {
            write!(f, "{}-{}", self.number, self.name)
        };
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    title: String,
    note: String,
    description: String,

    frequency: Frequency,
    last: String,

    #[serde(default = "types::default_string")]
    snap_to: String,

    #[serde(default = "types::default_zero_i32")]
    buffer_days: i32,

    #[serde(default = "types::default_vec")]
    subtasks: Vec<Subtask>,

    #[serde(default = "types::default_true")]
    active: bool,
}

pub(crate) fn load_one(file_path: &Path, task_data: &mut dyn TaskAddable) {
    let data: Data = match types::load(file_path) {
        None => {
            return;
        }
        Some(data) => data,
    };

    let last_date = match NaiveDate::parse_from_str(data.last.as_str(), "%Y-%m-%d") {
        Err(_) => {
            logging::error(format!(
                "Failed to convert last date in recurring task: '{}' ({})",
                data.last, data.title
            ));
            return;
        }
        Ok(date) => date,
    };

    if data.frequency.number < 1 {
        logging::error(format!(
            "Invalid frequency number: '{}' ({})",
            data.frequency.number, data.title
        ));
        return;
    }

    let task_date_option: Option<NaiveDate> = match data.frequency.name.as_str() {
        "year" => time::add_years(&last_date, data.frequency.number),
        "month" => time::add_months(&last_date, data.frequency.number),
        "week" => time::add_weeks(&last_date, data.frequency.number),
        "day" => time::add_days(&last_date, data.frequency.number),
        _ => None,
    };

    let mut task_date: NaiveDate = match task_date_option {
        None => {
            logging::error(format!("Unable to parse task frequency ({})", data.title));
            return;
        }
        Some(date) => date,
    };

    let today: NaiveDate = time::today();

    match data.snap_to.as_str() {
        "none" | "" => {}
        "today" => {
            if task_date < today {
                task_date = today;
            }
        }
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun" => {
            if task_date < today {
                task_date = today;
            }
            while !time::weekday_abbrev(&task_date).eq(&data.snap_to) {
                task_date = time::increment_by_one_day(&task_date);
            }
        }
        _ => {
            logging::error(format!(
                "Unable to parse task snap_to: '{}' ({})",
                data.snap_to, data.title
            ));
            return;
        }
    };

    if data.buffer_days != 0 {
        task_date = time::adjust_by_buffer_days(&task_date, data.buffer_days)
            .expect("Failed to subtract day.");
    }

    let task: Task = Task {
        meta: TaskMeta {
            frequency: format!("{}", data.frequency),
            subtasks: data.subtasks,
        },
        contents: TaskContents {
            title: data.title,
            note: data.note,
            active: data.active,
        },
    };

    task_data.add_task(task_date, task);
}
