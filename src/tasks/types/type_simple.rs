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

pub(crate) const DIR_NAME: &str = "simple";

#[derive(Serialize, Deserialize)]
struct SimpleItem {
    title: String,
    note: String,
    due: String,
    done: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    prefix: String,
    description: String,
    items: Vec<SimpleItem>,
}

pub(crate) fn load(file_path: &Path, task_data: &mut dyn TaskAddable) {
    let data: Data = match types::load(file_path) {
        None => {
            return;
        }
        Some(data) => data,
    };

    for item in data.items {
        if !item.done.is_empty() {
            continue;
        }

        let due_date = match NaiveDate::parse_from_str(item.due.as_str(), "%Y-%m-%d") {
            Err(_) => {
                logging::error(format!(
                    "Failed to convert due date in simple task: '{}' ({})",
                    item.due, item.title
                ));
                return;
            }
            Ok(date) => date,
        };

        let is_done: bool = !item.done.is_empty();
        let visibility: TaskVisibility = if is_done {
            TaskVisibility::Hidden
        } else {
            TaskVisibility::Visible
        };

        let task: Task = Task {
            meta: TaskMeta {
                frequency: data.prefix.clone(),
                subtasks: Default::default(),
            },
            contents: TaskContents {
                title: item.title,
                note: item.note,
                is_done,
                visibility,
            },
        };

        task_data.add_task(due_date, task);
    }
}
