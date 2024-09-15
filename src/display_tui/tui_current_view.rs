// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Clone)]
pub(super) enum CurrentView {
    Overdue,
    Today,
    RestOfTheWeek,
    LaterAndOther,
}

impl Default for CurrentView {
    fn default() -> Self {
        return CurrentView::Today;
    }
}

impl CurrentView {
    pub(super) fn prev(&self) -> Self {
        return match self {
            CurrentView::Overdue => self.clone(),
            CurrentView::Today => CurrentView::Overdue,
            CurrentView::RestOfTheWeek => CurrentView::Today,
            CurrentView::LaterAndOther => CurrentView::RestOfTheWeek,
        };
    }

    pub(super) fn next(&self) -> Self {
        return match self {
            CurrentView::Overdue => CurrentView::Today,
            CurrentView::Today => CurrentView::RestOfTheWeek,
            CurrentView::RestOfTheWeek => CurrentView::LaterAndOther,
            CurrentView::LaterAndOther => self.clone(),
        };
    }
}
