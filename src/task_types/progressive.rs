// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use chrono::{Datelike, DateTime, Local, NaiveDate};

use crate::tasks::Task;


pub(crate) const DIR_NAME: &str = "progressive";


pub(crate) fn parse(file_path: &PathBuf) -> (NaiveDate, Task) {
    let timestamp: DateTime<Local> = Local::now();
    return (
        NaiveDate::from_ymd_opt(
            timestamp.year(),
            timestamp.month(),
            timestamp.day()
        ).expect("Failed to create NaiveDate"),
        Task {
            frequency: String::from("frequency"),
            title: String::from(
                file_path.file_name()
                    .expect("Failed to parse filename.")
                    .to_str()
                    .expect("Failed to convert string.")
            ),
            note: String::from("note"),
        },
    )
}
