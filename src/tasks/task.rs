// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

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
