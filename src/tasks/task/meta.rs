// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
// dependencies
use serde::{Deserialize, Serialize};
// internal
use crate::tasks::task::contents::TaskContents;

pub(crate) struct TaskMeta {
    pub(crate) frequency: String,
    pub(crate) time_of_day: TaskTimeOfDay,
    pub(crate) subtasks: Vec<TaskContents>,
}

impl fmt::Display for TaskMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let frequency_display: String = if self.frequency.is_empty() {
            "".to_string()
        } else {
            format!("{} - ", self.frequency)
        };

        let time_of_day_mark: &str = match self.time_of_day {
            TaskTimeOfDay::Any => "",
            TaskTimeOfDay::Morning => "M",
            TaskTimeOfDay::Midday => "D",
            TaskTimeOfDay::Evening => "E",
        };
        let time_of_day_display: String = match self.time_of_day {
            TaskTimeOfDay::Any => "".to_string(),
            _ => {
                format!("({}) ", time_of_day_mark)
            }
        };

        let display: String = format!("{}{}", frequency_display, time_of_day_display);
        return if display.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{}", display)
        };
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub(crate) enum TaskTimeOfDay {
    Morning,
    Midday,
    Any,
    Evening,
}

impl Default for TaskTimeOfDay {
    fn default() -> Self {
        return TaskTimeOfDay::Any;
    }
}
