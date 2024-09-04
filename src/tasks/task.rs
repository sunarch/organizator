// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;
use std::fmt;
// internal
use crate::tasks::task_contents::TaskContents;
use crate::tasks::task_meta::TaskMeta;

pub struct Task {
    pub meta: TaskMeta,
    pub contents: TaskContents,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}{}", self.meta, self.contents);
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self
            .contents
            .title
            .to_ascii_lowercase()
            .cmp(&other.contents.title.to_ascii_lowercase())
        {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self
                .meta
                .frequency
                .to_ascii_lowercase()
                .cmp(&other.meta.frequency.to_ascii_lowercase()),
        };
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
            && self.contents.title == other.contents.title
            && self.contents.note == other.contents.note
            && self.contents.active == other.contents.active
    }
}
impl Eq for Task {}
