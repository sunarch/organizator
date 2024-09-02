// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde;
use serde_json;
// internal
use crate::logging;
use crate::tasks::task::Task;

pub type FnParse = dyn Fn(&Path) -> Option<(NaiveDate, Task)>;

pub fn default_true() -> bool {
    return true;
}

pub fn default_zero_i32() -> i32 {
    return 0;
}

pub fn default_string() -> String {
    return Default::default();
}

pub fn default_vec<T>() -> Vec<T> {
    return Default::default();
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Subtask {
    pub(crate) title: String,
    pub(crate) done: String,
}

pub(crate) fn load<Data: for<'de> serde::Deserialize<'de>>(file_path: &Path) -> Option<Data> {
    let file = match File::open(file_path) {
        Err(why) => {
            logging::error(format!(
                "Couldn't open todo file '{}' \n{}",
                file_path.display(),
                why
            ));
            return None;
        }
        Ok(file) => file,
    };

    match serde_json::from_reader(file) {
        Err(why) => {
            logging::error(format!(
                "Couldn't parse todo file '{}' \n{}",
                file_path.display(),
                why
            ));
            return None;
        }
        Ok(data) => Some(data),
    }
}
