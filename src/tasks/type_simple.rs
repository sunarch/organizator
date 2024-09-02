// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use crate::logging;
use chrono::NaiveDate;
use serde;
// internal
use crate::tasks::task::Task;
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

pub(crate) fn parse(file_path: &Path) -> Option<(NaiveDate, Task)> {
    let data: Data = match types::load(file_path) {
        None => {
            return None;
        }
        Some(data) => data,
    };

    if !data.done.is_empty() {
        return None;
    }

    let due_date = match NaiveDate::parse_from_str(data.due.as_str(), "%Y-%m-%d") {
        Err(_) => {
            logging::error(format!(
                "Failed to convert due date in simple task: '{}' ({})",
                data.due, data.title
            ));
            return None;
        }
        Ok(date) => date,
    };

    return Some((
        due_date,
        Task {
            frequency: data.prefix,
            title: data.title,
            note: data.note,
            subtasks: Default::default(),
            active: data.done.is_empty(),
        },
    ));
}
