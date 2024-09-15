// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod dated;
mod tui_current_view;
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
use crate::display_tui::tui_current_view::CurrentView;
use crate::display_tui::tui_view::TuiView;
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
    current_view: CurrentView,

    view_overdue: TuiView,
    view_today: TuiView,
    view_rest_of_the_week: TuiView,
    view_later_and_other: TuiView,
}

impl Tui {
    fn new() -> Self {
        return Self {
            current_view: Default::default(),

            view_overdue: Default::default(),
            view_today: Default::default(),
            view_rest_of_the_week: Default::default(),
            view_later_and_other: Default::default(),
        };
    }

    fn scroll(&mut self, direction: ScrollDirection) {
        let view: &mut TuiView = match self.current_view {
            CurrentView::Overdue => &mut self.view_overdue,
            CurrentView::Today => &mut self.view_today,
            CurrentView::RestOfTheWeek => &mut self.view_rest_of_the_week,
            CurrentView::LaterAndOther => &mut self.view_later_and_other,
        };
        match direction {
            ScrollDirection::Forward => {
                view.vertical_scroll = view
                    .content_length
                    .min(view.vertical_scroll.saturating_add(1));
            }
            ScrollDirection::Backward => {
                view.vertical_scroll = view.vertical_scroll.saturating_sub(1);
            }
        }
        view.scrollbar_state = view.scrollbar_state.position(view.vertical_scroll);
    }

    fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        task_data: &TaskData,
    ) -> Result<(), io::Error> {
        let (par_of_overdue, len_of_overdue) = dated::par_of_overdue(task_data);
        self.view_overdue = TuiView::new(len_of_overdue);

        let (par_of_today, len_of_today) = dated::par_of_today(task_data);
        self.view_today = TuiView::new(len_of_today);

        let (par_of_rest_of_the_week, len_of_rest_of_the_week) =
            dated::par_of_rest_of_the_week(task_data);
        self.view_rest_of_the_week = TuiView::new(len_of_rest_of_the_week);

        let (par_of_later_and_other, len_of_later_and_other) =
            dated::par_of_later_and_other(task_data);
        self.view_later_and_other = TuiView::new(len_of_later_and_other);

        loop {
            terminal.draw(|frame: &mut Frame| {
                let area: Rect = frame.area();

                match self.current_view {
                    CurrentView::Overdue => {
                        render_screen(
                            frame,
                            area,
                            &par_of_overdue,
                            self.view_overdue.vertical_scroll,
                        );
                    }
                    CurrentView::Today => {
                        render_screen(frame, area, &par_of_today, self.view_today.vertical_scroll);
                    }
                    CurrentView::RestOfTheWeek => {
                        render_screen(
                            frame,
                            area,
                            &par_of_rest_of_the_week,
                            self.view_rest_of_the_week.vertical_scroll,
                        );
                    }
                    CurrentView::LaterAndOther => {
                        render_screen(
                            frame,
                            area,
                            &par_of_later_and_other,
                            self.view_later_and_other.vertical_scroll,
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
                                self.current_view = CurrentView::Today;
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
                                self.current_view = CurrentView::Overdue;
                            }
                            KeyCode::Char('2') => {
                                self.current_view = CurrentView::Today;
                            }
                            KeyCode::Char('3') => {
                                self.current_view = CurrentView::RestOfTheWeek;
                            }
                            KeyCode::Char('4') => {
                                self.current_view = CurrentView::LaterAndOther;
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
            CurrentView::Overdue => &mut self.view_overdue.scrollbar_state,
            CurrentView::Today => &mut self.view_today.scrollbar_state,
            CurrentView::RestOfTheWeek => &mut self.view_rest_of_the_week.scrollbar_state,
            CurrentView::LaterAndOther => &mut self.view_later_and_other.scrollbar_state,
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
