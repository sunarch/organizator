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
        let mut display: String = Default::default();

        if self.time_of_day != TaskTimeOfDay::Any {
            display = format!("{} ({})", display, self.time_of_day);
        }

        if self.frequency.interval != TaskFrequencyInterval::None {
            display = format!("{} {} - ", display, self.frequency);
        }

        return write!(f, "{}", display.trim_start());
    }
}

impl TaskMeta {
    pub(crate) fn format_as_table_row(&self) -> String {
        let frequency_number_display = match self.frequency.number {
            None => " ".repeat(3),
            Some(1) => " ".repeat(3),
            Some(number) => format!("{: >2}-", number),
        };
        let frequency_interval_display = match self.frequency.number {
            None => " ".repeat(7),
            Some(1) => format!("{: <7}", self.frequency.interval.format_frequency_one()),
            Some(_) => format!("{: <7}", format!("{}  ", self.frequency.interval)),
        };

        return format!(
            "|{}|{}{}|",
            self.time_of_day, frequency_number_display, frequency_interval_display
        );
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
            Some(1) => write!(f, "{}", self.interval.format_frequency_one()),
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

impl TaskFrequencyInterval {
    fn format_frequency_one(&self) -> String {
        return match self {
            TaskFrequencyInterval::Other(text) => text.clone(),
            TaskFrequencyInterval::None => "".to_string(),
            TaskFrequencyInterval::Day => "daily".to_string(),
            _ => format!("{}ly", self),
        };
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
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

impl fmt::Display for TaskTimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time_of_day_mark: &str = match self {
            TaskTimeOfDay::Any => " ",
            TaskTimeOfDay::Morning => "M",
            TaskTimeOfDay::Midday => "D",
            TaskTimeOfDay::Evening => "E",
        };
        write!(f, "{}", time_of_day_mark)
    }
}
