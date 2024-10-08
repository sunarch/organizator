// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod contents;
pub(crate) mod meta;

use std::cmp::Ordering;
use std::fmt;
// internal
use crate::tasks::task::contents::TaskContents;
use crate::tasks::task::meta::{TaskMeta, TaskTimeOfDay};

pub(crate) struct Task {
    pub(crate) meta: TaskMeta,
    pub(crate) contents: TaskContents,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}{}", self.meta, self.contents);
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.meta.time_of_day, &other.meta.time_of_day) {
            (TaskTimeOfDay::Morning, _) => return Ordering::Less,
            (_, TaskTimeOfDay::Morning) => return Ordering::Greater,
            (TaskTimeOfDay::Evening, _) => return Ordering::Greater,
            (_, TaskTimeOfDay::Evening) => return Ordering::Less,
            (TaskTimeOfDay::Midday, TaskTimeOfDay::Any) => return Ordering::Less,
            (TaskTimeOfDay::Any, TaskTimeOfDay::Midday) => return Ordering::Greater,
            (_, _) => {}
        }

        match &self
            .meta
            .frequency
            .interval
            .cmp(&other.meta.frequency.interval)
        {
            Ordering::Equal => {
                let frequency_number_ordering: &Ordering =
                    &self.meta.frequency.number.cmp(&other.meta.frequency.number);
                if *frequency_number_ordering != Ordering::Equal {
                    return *frequency_number_ordering;
                }
            }
            decided_value => return *decided_value,
        }

        {
            let ordering: Ordering = self
                .contents
                .title
                .to_ascii_lowercase()
                .cmp(&other.contents.title.to_ascii_lowercase());
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        {
            let ordering: Ordering = self
                .contents
                .note
                .to_ascii_lowercase()
                .cmp(&other.contents.note.to_ascii_lowercase());
            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        return self.contents.is_done.cmp(&other.contents.is_done);
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.meta.frequency == other.meta.frequency
            && self.meta.time_of_day == other.meta.time_of_day
            && self.contents.title == other.contents.title
            && self.contents.note == other.contents.note
            && self.contents.is_done == other.contents.is_done
    }
}
impl Eq for Task {}
