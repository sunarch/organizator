// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
// internal
use crate::logging;
use crate::tasks::data::TaskAddable;
use crate::tasks::task::contents::{TaskContents, TaskVisibility};
use crate::tasks::task::meta::{
    TaskFrequency, TaskFrequencyInterval, TaskMeta, TaskMetaDisplayOptions, TaskTimeOfDay,
};
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
    pivot: Option<DataPivot>,

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
}

#[derive(Serialize, Deserialize)]
struct DataPivot {
    weekday: Option<DataWeekday>,
}

#[derive(Serialize, Deserialize)]
enum DataWeekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl DataWeekday {
    fn to_chrono_weekday(&self) -> Weekday {
        return match self {
            DataWeekday::Mon => Weekday::Mon,
            DataWeekday::Tue => Weekday::Tue,
            DataWeekday::Wed => Weekday::Wed,
            DataWeekday::Thu => Weekday::Thu,
            DataWeekday::Fri => Weekday::Fri,
            DataWeekday::Sat => Weekday::Sat,
            DataWeekday::Sun => Weekday::Sun,
        };
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

    if let Some(pivot) = data.pivot {
        if let Some(weekday) = pivot.weekday {
            while task_date.weekday() != weekday.to_chrono_weekday() {
                task_date = time::increment_by_one_day(&task_date);
            }
        }
    }

    if data.buffer_days != 0 {
        task_date = time::adjust_by_buffer_days(&task_date, data.buffer_days)
            .expect("Failed to subtract day.");
    }

    let overdue: bool = task_date < today;

    if let Some(snap_to) = data.snap_to {
        match snap_to {
            DataSnapTo::Today => {
                if task_date < today {
                    task_date = today;
                }
            }
            DataSnapTo::ToBeDetermined => {}
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
            overdue,
            subtasks,
            display_options: TaskMetaDisplayOptions {
                overdue_mark: task_date == today && data.active,
            },
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
