// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
// internal
use crate::tasks::data::TaskAddable;
use crate::tasks::task::contents::{TaskContents, TaskVisibility};
use crate::tasks::task::meta::{TaskFrequency, TaskFrequencyInterval, TaskMeta, TaskTimeOfDay};
use crate::tasks::task::Task;
use crate::tasks::types;
use crate::{logging, time};

pub(crate) const DIR_NAME: &str = "progressive";

#[derive(Serialize, Deserialize)]
struct Data {
    title: String,
    description: Option<String>,
    days: Vec<DataDay>,
}

#[derive(Serialize, Deserialize)]
struct DataDay {
    title: String,
    items: Vec<DataItem>,
}

#[derive(Serialize, Deserialize)]
struct DataItem {
    done: String,

    #[serde(default = "TaskTimeOfDay::default")]
    time_of_day: TaskTimeOfDay,
}

pub(crate) fn load_one(file_path: &Path, task_data: &mut dyn TaskAddable) {
    let data: Data = match types::load(file_path) {
        None => {
            return;
        }
        Some(data) => data,
    };

    if data.days.is_empty() {
        logging::error(format!("No days in progressive task  ({})", data.title));
        return;
    }

    let mut previous_day_last_date_string_opt: Option<&String> = None;
    let mut current_day_opt: Option<&DataDay> = None;

    for day in &data.days {
        if day.items.is_empty() {
            logging::error(format!(
                "No items in progressive task day: '{}' ({})",
                day.title, data.title
            ));
            return;
        }

        if day
            .items
            .iter()
            .map(|item| item.done.is_empty())
            .any(|is_not_done| is_not_done)
        {
            current_day_opt = Some(day);
            break;
        } else {
            match day.items.last() {
                None => unreachable!(),
                Some(item) => {
                    previous_day_last_date_string_opt = Some(&item.done);
                }
            }
        }
    }

    let mut last_date_opt: Option<NaiveDate> = match previous_day_last_date_string_opt {
        None => None,
        Some(last_date_string) => time::parsing::date_opt_from_str(
            last_date_string,
            "progressive task",
            data.title.as_str(),
        ),
    };
    let mut all_done_for_current_day: bool = true; // default, changed below
    let current_day: &DataDay = match current_day_opt {
        None => {
            // no items with empty done: all items done
            return;
        }
        Some(day) => {
            if let Some(last_date) = search_for_last_date(&day.items, &data.title) {
                last_date_opt = Some(last_date);
                all_done_for_current_day = false;
            }
            day
        }
    };

    let mut task_date: NaiveDate = task_data.date_today();
    match last_date_opt {
        None => {}
        Some(last_date) => {
            if last_date == task_date && all_done_for_current_day {
                task_date = task_data.date_tomorrow();
            }
        }
    }

    for item in &current_day.items {
        let task: Task = Task {
            meta: TaskMeta {
                frequency: TaskFrequency {
                    number: None,
                    interval: TaskFrequencyInterval::Other("(PR)".to_string()),
                },
                time_of_day: item.time_of_day.clone(),
                subtasks: Default::default(),
            },
            contents: TaskContents {
                title: data.title.clone(),
                note: current_day.title.clone(),
                is_done: !item.done.is_empty(),
                visibility: TaskVisibility::Visible,
            },
        };

        task_data.add_task(task_date, task);
    }
}

fn search_for_last_date(items: &Vec<DataItem>, note_item: &str) -> Option<NaiveDate> {
    let mut last_date_string_search: Option<&String> = None;
    for item in items {
        if item.done.is_empty() {
            break;
        } else {
            last_date_string_search = Some(&item.done);
        }
    }
    return match last_date_string_search {
        None => None,
        Some(text) => time::parsing::date_opt_from_str(text, "progressive task", note_item),
    };
}
