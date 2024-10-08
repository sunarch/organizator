// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

pub(crate) struct TaskContents {
    pub(crate) title: String,
    pub(crate) note: String,
    pub(crate) is_done: bool,
    pub(crate) visibility: TaskVisibility,
}

#[derive(PartialEq)]
pub(crate) enum TaskVisibility {
    Visible,
    Inactive,
    Hidden,
}

impl fmt::Display for TaskContents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note_display: String = if self.note.is_empty() {
            "".to_string()
        } else {
            format!(" ({})", self.note)
        };
        return write!(f, "{}{}", self.title, note_display);
    }
}
