// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};
use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
// internal
use crate::logging;
use crate::tasks::data::TaskAddable;
use crate::tasks::task::contents::{TaskContents, TaskVisibility};
use crate::tasks::task::meta::{TaskFrequency, TaskFrequencyInterval, TaskMeta, TaskTimeOfDay};
use crate::tasks::task::Task;
use crate::tasks::types;
use crate::time;

pub(crate) const DIR_NAME: &str = "recurring";

#[derive(Serialize, Deserialize)]
struct Data {
    title: String,
    note: String,
    description: Option<String>,

    frequency: TaskFrequency,
    last: String,

    snap_to: Option<DataSnapTo>,

    #[serde(default = "TaskTimeOfDay::default")]
    time_of_day: TaskTimeOfDay,

    #[serde(default = "types::default_zero_i32")]
    buffer_days: i32,

    #[serde(default = "types::default_vec")]
    subtasks: Vec<DataSubtask>,

    #[serde(default = "types::default_true")]
    active: bool,

    #[serde(default = "types::default_false")]
    pub(crate) hidden: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DataSubtask {
    pub(crate) title: String,
    pub(crate) done: String,

    #[serde(default = "types::default_false")]
    pub(crate) hidden: bool,
}

#[derive(Serialize, Deserialize)]
enum DataSnapTo {
    ToBeDetermined,
    Today,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Display for DataSnapTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display: &str = match self {
            DataSnapTo::ToBeDetermined => "TBD",
            DataSnapTo::Today => "Today",
            DataSnapTo::Mon => "Mon",
            DataSnapTo::Tue => "Tue",
            DataSnapTo::Wed => "Wed",
            DataSnapTo::Thu => "Thu",
            DataSnapTo::Fri => "Fri",
            DataSnapTo::Sat => "Sat",
            DataSnapTo::Sun => "Sun",
        };
        return write!(f, "{}", display);
    }
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

    let frequency_number: u8 = match data.frequency.number {
        None => {
            logging::error(format!("Missing frequency number ({})", data.title));
            return;
        }
        Some(number) => number,
    };

    if frequency_number < 1 {
        logging::error(format!("Frequency number cannot be zero ({})", data.title));
        return;
    }

    let task_date_option: Option<NaiveDate> = match data.frequency.interval {
        TaskFrequencyInterval::Other(_) => None,
        TaskFrequencyInterval::Day => time::add_days(&last_date, frequency_number),
        TaskFrequencyInterval::Week => time::add_weeks(&last_date, frequency_number),
        TaskFrequencyInterval::Month => time::add_months(&last_date, frequency_number),
        TaskFrequencyInterval::Year => time::add_years(&last_date, frequency_number),
        TaskFrequencyInterval::None => None,
    };

    let mut task_date: NaiveDate = match task_date_option {
        None => {
            logging::error(format!("Unable to parse task frequency ({})", data.title));
            return;
        }
        Some(date) => date,
    };

    let today: NaiveDate = task_data.date_today();

    if data.buffer_days != 0 {
        task_date = time::adjust_by_buffer_days(&task_date, data.buffer_days)
            .expect("Failed to subtract day.");
    }

    if let Some(snap_to) = data.snap_to {
        match snap_to {
            DataSnapTo::ToBeDetermined => {}
            DataSnapTo::Today => {
                if task_date < today {
                    task_date = today;
                }
            }
            DataSnapTo::Mon
            | DataSnapTo::Tue
            | DataSnapTo::Wed
            | DataSnapTo::Thu
            | DataSnapTo::Fri
            | DataSnapTo::Sat
            | DataSnapTo::Sun => {
                if task_date < today {
                    task_date = today;
                }
                while !time::weekday_abbrev(&task_date).eq(&format!("{}", snap_to)) {
                    task_date = time::increment_by_one_day(&task_date);
                }
            }
        }
    }

    let subtasks: Vec<TaskContents> = data
        .subtasks
        .iter()
        .map(|subtask| TaskContents {
            title: subtask.title.clone(),
            note: "".to_string(),
            is_done: !subtask.done.is_empty(),
            visibility: if subtask.hidden {
                TaskVisibility::Hidden
            } else {
                TaskVisibility::Visible
            },
        })
        .collect();

    let mut task_visibility: TaskVisibility = TaskVisibility::Visible;
    if !data.active {
        task_visibility = TaskVisibility::Inactive;
    }
    if data.hidden {
        task_visibility = TaskVisibility::Hidden;
    }

    let task: Task = Task {
        meta: TaskMeta {
            frequency: data.frequency,
            time_of_day: data.time_of_day,
            subtasks,
        },
        contents: TaskContents {
            title: data.title,
            note: data.note,
            is_done: false,
            visibility: task_visibility,
        },
    };

    task_data.add_task(task_date, task);
}
