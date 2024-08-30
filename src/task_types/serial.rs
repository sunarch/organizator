// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::{DateTime, Datelike, Local, NaiveDate};
// internal
use crate::tasks::Task;

pub(crate) const DIR_NAME: &str = "serial";

pub(crate) fn parse(file_path: &Path) -> (NaiveDate, Task) {
    let timestamp: DateTime<Local> = Local::now();
    return (
        NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), timestamp.day())
            .expect("Failed to create NaiveDate"),
        Task {
            frequency: String::from("frequency"),
            title: String::from(
                file_path
                    .file_name()
                    .expect("Failed to parse filename.")
                    .to_str()
                    .expect("Failed to convert string."),
            ),
            note: String::from("note"),
        },
    );
}
