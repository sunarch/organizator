// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Clone)]
pub(super) enum View {
    Overdue,
    Today,
    RestOfTheWeek,
    LaterAndOther,
}

impl Default for View {
    fn default() -> Self {
        return View::Today;
    }
}

impl View {
    pub(super) fn prev(&self) -> Self {
        return match self {
            View::Overdue => self.clone(),
            View::Today => View::Overdue,
            View::RestOfTheWeek => View::Today,
            View::LaterAndOther => View::RestOfTheWeek,
        };
    }

    pub(super) fn next(&self) -> Self {
        return match self {
            View::Overdue => View::Today,
            View::Today => View::RestOfTheWeek,
            View::RestOfTheWeek => View::LaterAndOther,
            View::LaterAndOther => self.clone(),
        };
    }
}
