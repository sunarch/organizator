// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use ratatui::widgets::ScrollbarState;

const INITIAL_SCROLL: usize = 0;

pub(super) struct TuiView {
    pub(super) content_length: usize,
    pub(super) vertical_scroll: usize,
    pub(super) scrollbar_state: ScrollbarState,
}

impl Default for TuiView {
    fn default() -> Self {
        return Self {
            content_length: Default::default(),
            vertical_scroll: Default::default(),
            scrollbar_state: Default::default(),
        };
    }
}

impl TuiView {
    pub(super) fn new(content_length: usize) -> Self {
        return Self {
            content_length,
            vertical_scroll: INITIAL_SCROLL,
            scrollbar_state: ScrollbarState::new(content_length).position(INITIAL_SCROLL),
        };
    }
}
