// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod dated;
mod tui_view;

use std::io;
use std::time::Duration;
// dependencies
use ratatui::layout::Margin;
use ratatui::widgets::{
    Paragraph, ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Rect,
    DefaultTerminal, Frame,
};
// internal
use crate::display_tui::tui_view::View;
use crate::logging;
use crate::tasks::data::TaskData;

const EVENT_POLL_TIMEOUT: Duration = Duration::from_millis(16);

pub(crate) fn run(task_data: &TaskData) -> Result<(), io::Error> {
    logging::info("Running TUI ...".to_string());
    let terminal: DefaultTerminal = ratatui::init();
    let tui_result: Result<(), io::Error> = Tui::new().run(terminal, task_data);
    ratatui::restore();
    logging::info("Exiting TUI ...".to_string());
    tui_result
}

struct Tui {
    current_view: View,

    vertical_scroll_of_overdue: usize,
    scrollbar_state_of_overdue: ScrollbarState,

    vertical_scroll_of_today: usize,
    scrollbar_state_of_today: ScrollbarState,

    vertical_scroll_of_rest_of_the_week: usize,
    scrollbar_state_of_rest_of_the_week: ScrollbarState,

    vertical_scroll_of_later_and_other: usize,
    scrollbar_state_of_later_and_other: ScrollbarState,
}

impl Tui {
    fn new() -> Self {
        return Self {
            current_view: Default::default(),

            vertical_scroll_of_overdue: 0,
            scrollbar_state_of_overdue: Default::default(),

            vertical_scroll_of_today: 0,
            scrollbar_state_of_today: Default::default(),

            vertical_scroll_of_rest_of_the_week: 0,
            scrollbar_state_of_rest_of_the_week: Default::default(),

            vertical_scroll_of_later_and_other: 0,
            scrollbar_state_of_later_and_other: Default::default(),
        };
    }

    fn scroll(&mut self, direction: ScrollDirection) {
        let (vertical_scroll, scrollbar_state) = match self.current_view {
            View::Overdue => (
                &mut self.vertical_scroll_of_overdue,
                &mut self.scrollbar_state_of_overdue,
            ),
            View::Today => (
                &mut self.vertical_scroll_of_today,
                &mut self.scrollbar_state_of_today,
            ),
            View::RestOfTheWeek => (
                &mut self.vertical_scroll_of_rest_of_the_week,
                &mut self.scrollbar_state_of_rest_of_the_week,
            ),
            View::LaterAndOther => (
                &mut self.vertical_scroll_of_later_and_other,
                &mut self.scrollbar_state_of_later_and_other,
            ),
        };
        match direction {
            ScrollDirection::Forward => {
                *vertical_scroll = vertical_scroll.saturating_add(1);
            }
            ScrollDirection::Backward => {
                *vertical_scroll = vertical_scroll.saturating_sub(1);
            }
        }
        *scrollbar_state = scrollbar_state.position(*vertical_scroll);
    }

    fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        task_data: &TaskData,
    ) -> Result<(), io::Error> {
        let (par_of_overdue, len_of_overdue) = dated::par_of_overdue(task_data);
        self.scrollbar_state_of_overdue =
            ScrollbarState::new(len_of_overdue).position(self.vertical_scroll_of_overdue);

        let (par_of_today, len_of_today) = dated::par_of_today(task_data);
        self.scrollbar_state_of_today =
            ScrollbarState::new(len_of_today).position(self.vertical_scroll_of_today);

        let (par_of_rest_of_the_week, len_of_rest_of_the_week) =
            dated::par_of_rest_of_the_week(task_data);
        self.scrollbar_state_of_rest_of_the_week = ScrollbarState::new(len_of_rest_of_the_week)
            .position(self.vertical_scroll_of_rest_of_the_week);

        let (par_of_later_and_other, len_of_later_and_other) =
            dated::par_of_later_and_other(task_data);
        self.scrollbar_state_of_later_and_other = ScrollbarState::new(len_of_later_and_other)
            .position(self.vertical_scroll_of_later_and_other);

        loop {
            terminal.draw(|frame: &mut Frame| {
                let area: Rect = frame.area();

                match self.current_view {
                    View::Overdue => {
                        render_screen(
                            frame,
                            area,
                            &par_of_overdue,
                            self.vertical_scroll_of_overdue,
                        );
                    }
                    View::Today => {
                        render_screen(frame, area, &par_of_today, self.vertical_scroll_of_today);
                    }
                    View::RestOfTheWeek => {
                        render_screen(
                            frame,
                            area,
                            &par_of_rest_of_the_week,
                            self.vertical_scroll_of_rest_of_the_week,
                        );
                    }
                    View::LaterAndOther => {
                        render_screen(
                            frame,
                            area,
                            &par_of_later_and_other,
                            self.vertical_scroll_of_later_and_other,
                        );
                    }
                }

                self.render_scrollbar(frame, area);
            })?;

            if event::poll(EVENT_POLL_TIMEOUT)? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('t') => {
                                self.current_view = View::Today;
                            }
                            KeyCode::Char('q') => break,
                            KeyCode::Char('j') | KeyCode::Down => {
                                self.scroll(ScrollDirection::Forward);
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                self.scroll(ScrollDirection::Backward);
                            }
                            KeyCode::Char('h') | KeyCode::Left => {
                                self.current_view = self.current_view.prev();
                            }
                            KeyCode::Char('l') | KeyCode::Right => {
                                self.current_view = self.current_view.next();
                            }
                            KeyCode::Char('1') => {
                                self.current_view = View::Overdue;
                            }
                            KeyCode::Char('2') => {
                                self.current_view = View::Today;
                            }
                            KeyCode::Char('3') => {
                                self.current_view = View::RestOfTheWeek;
                            }
                            KeyCode::Char('4') => {
                                self.current_view = View::LaterAndOther;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        return Ok(());
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        let scrollbar_state: &mut ScrollbarState = match self.current_view {
            View::Overdue => &mut self.scrollbar_state_of_overdue,
            View::Today => &mut self.scrollbar_state_of_today,
            View::RestOfTheWeek => &mut self.scrollbar_state_of_rest_of_the_week,
            View::LaterAndOther => &mut self.scrollbar_state_of_later_and_other,
        };

        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            scrollbar_state,
        );
    }
}

fn render_screen(frame: &mut Frame, area: Rect, par_of_screen: &Paragraph, vertical_scroll: usize) {
    frame.render_widget(
        par_of_screen.clone().scroll((vertical_scroll as u16, 0)),
        area,
    );
}
