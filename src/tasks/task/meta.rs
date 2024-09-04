// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
// internal
use crate::tasks::task::contents::TaskContents;

pub struct TaskMeta {
    pub frequency: String,
    pub subtasks: Vec<TaskContents>,
}

impl fmt::Display for TaskMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let frequency_display: String = if self.frequency.is_empty() {
            "".to_string()
        } else {
            format!("{} - ", self.frequency)
        };
        return write!(f, "{}", frequency_display);
    }
}
