// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde;
// internal
use crate::logging;
use crate::tasks::task::Task;
use crate::tasks::types;
use crate::time;

pub(crate) const DIR_NAME: &str = "progressive";

#[derive(serde::Serialize, serde::Deserialize)]
struct ProgressiveItem {
    title: String,
    done: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    title: String,
    description: String,
    items: Vec<ProgressiveItem>,
}

pub(crate) fn parse(file_path: &Path) -> Option<(NaiveDate, Task)> {
    let data: Data = match types::load(file_path) {
        None => {
            return None;
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
        return None;
    }

    let last_date = match NaiveDate::parse_from_str(last_date_string.as_str(), "%Y-%m-%d") {
        Err(_) => {
            logging::error(format!(
                "Failed to convert last date in progressive task: '{}' ({})",
                last_date_string, data.title
            ));
            return None;
        }
        Ok(date) => date,
    };

    let today: NaiveDate = time::today();
    let mut task_date: NaiveDate = today;
    if last_date == today {
        task_date = time::add_days(task_date, 1).expect("Failed to add day.");
    };

    return Some((
        task_date,
        Task {
            frequency: String::from("(PR)"),
            title: data.title,
            note: task_note,
            subtasks: Default::default(),
            active: true,
        },
    ));
}
