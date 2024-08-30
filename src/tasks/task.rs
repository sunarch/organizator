// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;
use std::fmt;

#[derive(Eq)]
pub struct Task {
    pub frequency: String,
    pub title: String,
    pub note: String,
    pub active: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        if !self.frequency.is_empty() {
            display = format!("{} - ", self.frequency);
        }
        display.push_str(self.title.as_str());
        if !self.note.is_empty() {
            display = format!("{} ({})", display, self.note);
        }
        return write!(f, "{}", display);
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self
            .title
            .to_ascii_lowercase()
            .cmp(&other.title.to_ascii_lowercase())
        {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self
                .frequency
                .to_ascii_lowercase()
                .cmp(&other.frequency.to_ascii_lowercase()),
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
        self.frequency == other.frequency
            && self.title == other.title
            && self.note == other.note
            && self.active == other.active
    }
}
