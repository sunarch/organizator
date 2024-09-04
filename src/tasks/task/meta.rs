// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
// dependencies
use serde::{Deserialize, Serialize};
// internal
use crate::tasks::task::contents::TaskContents;

pub(crate) struct TaskMeta {
    pub(crate) frequency: TaskFrequency,
    pub(crate) time_of_day: TaskTimeOfDay,
    pub(crate) subtasks: Vec<TaskContents>,
}

impl fmt::Display for TaskMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let frequency_display: String = match self.frequency.interval {
            TaskFrequencyInterval::None => "".to_string(),
            _ => format!("{} - ", self.frequency),
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

        let display: String = format!("{}{}", time_of_day_display, frequency_display);
        return if display.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{}", display)
        };
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TaskFrequency {
    pub(crate) number: Option<u8>,
    pub(crate) interval: TaskFrequencyInterval,
}

impl Default for TaskFrequency {
    fn default() -> Self {
        return TaskFrequency {
            number: Default::default(),
            interval: Default::default(),
        };
    }
}

impl fmt::Display for TaskFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match &self.number {
            None => write!(f, "{}", self.interval),
            Some(1) => write!(f, "{}ly", self.interval),
            Some(number) => write!(f, "{}-{}", number, self.interval),
        };
    }
}

impl PartialEq<Self> for TaskFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval && self.number == other.number
    }
}
impl Eq for TaskFrequency {}

#[derive(PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum TaskFrequencyInterval {
    Other(String),
    Day,
    Week,
    Month,
    Year,
    None,
}

impl Default for TaskFrequencyInterval {
    fn default() -> Self {
        return TaskFrequencyInterval::None;
    }
}

impl fmt::Display for TaskFrequencyInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let frequency_mark: &str = match self {
            TaskFrequencyInterval::Other(text) => text.as_str(),
            TaskFrequencyInterval::Day => "day",
            TaskFrequencyInterval::Week => "week",
            TaskFrequencyInterval::Month => "month",
            TaskFrequencyInterval::Year => "year",
            TaskFrequencyInterval::None => "",
        };
        return write!(f, "{}", frequency_mark);
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
