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
use crate::tasks::task_data::TaskAddable;
use crate::tasks::types;

pub(crate) const DIR_NAME: &str = "simple";

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    title: String,
    note: String,
    description: String,
    prefix: String,
    due: String,
    done: String,
}

pub(crate) fn load(file_path: &Path, task_data: &mut dyn TaskAddable) {
    let data: Data = match types::load(file_path) {
        None => {
            return;
        }
        Some(data) => data,
    };

    if !data.done.is_empty() {
        return;
    }

    let due_date = match NaiveDate::parse_from_str(data.due.as_str(), "%Y-%m-%d") {
        Err(_) => {
            logging::error(format!(
                "Failed to convert due date in simple task: '{}' ({})",
                data.due, data.title
            ));
            return;
        }
        Ok(date) => date,
    };

    let task: Task = Task {
        frequency: data.prefix,
        title: data.title,
        note: data.note,
        subtasks: Default::default(),
        active: data.done.is_empty(),
    };

    task_data.add_task(due_date, task);
}
