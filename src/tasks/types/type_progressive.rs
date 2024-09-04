// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
// internal
use crate::logging;
use crate::tasks::data::TaskAddable;
use crate::tasks::task::contents::{TaskContents, TaskVisibility};
use crate::tasks::task::meta::TaskMeta;
use crate::tasks::task::Task;
use crate::tasks::types;
use crate::time;

pub(crate) const DIR_NAME: &str = "progressive";

#[derive(Serialize, Deserialize)]
struct Data {
    title: String,
    description: String,
    items: Vec<DataItem>,
}

#[derive(Serialize, Deserialize)]
struct DataItem {
    title: String,
    done: String,
}

pub(crate) fn load_one(file_path: &Path, task_data: &mut dyn TaskAddable) {
    let data: Data = match types::load(file_path) {
        None => {
            return;
        }
        Some(data) => data,
    };

    let mut last_date_string: String = Default::default();
    let mut task_note: String = String::new();
    for item in data.items {
        if item.done.is_empty() {
            task_note = item.title;
            break;
        } else {
            last_date_string = item.done;
        }
    }
    if task_note.is_empty() {
        // no items with empty done: all items done
        return;
    }

    let last_date = match NaiveDate::parse_from_str(last_date_string.as_str(), "%Y-%m-%d") {
        Err(_) => {
            logging::error(format!(
                "Failed to convert last date in progressive task: '{}' ({})",
                last_date_string, data.title
            ));
            return;
        }
        Ok(date) => date,
    };

    let today: NaiveDate = time::today();
    let mut task_date: NaiveDate = today;
    if last_date == today {
        task_date = time::increment_by_one_day(&task_date);
    };

    let task: Task = Task {
        meta: TaskMeta {
            frequency: String::from("(PR)"),
            subtasks: Default::default(),
        },
        contents: TaskContents {
            title: data.title,
            note: task_note,
            is_done: false,
            visibility: TaskVisibility::Visible,
        },
    };

    task_data.add_task(task_date, task);
}
