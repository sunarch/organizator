// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use ratatui::widgets::ScrollbarState;

pub(super) struct TuiView {
    pub(super) vertical_scroll: usize,
    pub(super) scrollbar_state: ScrollbarState,
}

impl Default for TuiView {
    fn default() -> Self {
        return Self {
            vertical_scroll: Default::default(),
            scrollbar_state: Default::default(),
        };
    }
}
