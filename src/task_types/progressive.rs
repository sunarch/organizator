// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::{DateTime, Datelike, Local, NaiveDate};
use serde;
// internal
use crate::task_types::_task_types;
use crate::tasks::Task;

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
    let timestamp: DateTime<Local> = Local::now();

    let data: Data = match _task_types::load(file_path) {
        None => {
            return None;
        }
        Some(data) => data,
    };

    let mut task_note: String = String::new();
    for item in data.items {
        if item.done == "" {
            task_note = item.title;
            break;
        }
    }

    return Some((
        NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), timestamp.day())
            .expect("Failed to create NaiveDate"),
        Task {
            frequency: String::from("(PR)"),
            title: data.title,
            note: task_note,
        },
    ));
}
