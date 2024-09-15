// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod dated;
mod tui_view;

use std::io;
use std::time::Duration;
// dependencies
use ratatui::layout::Margin;
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
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
    let tui_result: Result<(), io::Error> = run_tui(terminal, task_data);
    ratatui::restore();
    logging::info("Exiting TUI ...".to_string());
    tui_result
}

fn run_tui(mut terminal: DefaultTerminal, task_data: &TaskData) -> Result<(), io::Error> {
    let mut current_view: View = Default::default();

    let (par_of_overdue, len_of_overdue) = dated::par_of_overdue(task_data);
    let mut vertical_scroll_of_overdue: usize = 0;
    let mut scrollbar_state_of_overdue: ScrollbarState =
        ScrollbarState::new(len_of_overdue).position(vertical_scroll_of_overdue);

    let (par_of_today, len_of_today) = dated::par_of_today(task_data);
    let mut vertical_scroll_of_today: usize = 0;
    let mut scrollbar_state_of_today: ScrollbarState =
        ScrollbarState::new(len_of_today).position(vertical_scroll_of_today);

    let (par_of_rest_of_the_week, len_of_rest_of_the_week) =
        dated::par_of_rest_of_the_week(task_data);
    let mut vertical_scroll_of_rest_of_the_week: usize = 0;
    let mut scrollbar_state_of_rest_of_the_week: ScrollbarState =
        ScrollbarState::new(len_of_rest_of_the_week).position(vertical_scroll_of_rest_of_the_week);

    let (par_of_later_and_other, len_of_later_and_other) = dated::par_of_later_and_other(task_data);
    let mut vertical_scroll_of_later_and_other: usize = 0;
    let mut scrollbar_state_of_later_and_other: ScrollbarState =
        ScrollbarState::new(len_of_later_and_other).position(vertical_scroll_of_later_and_other);

    loop {
        terminal.draw(|frame: &mut Frame| {
            let area: Rect = frame.area();
            let area_inner: Rect = area.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            });
            match current_view {
                View::Overdue => {
                    render_screen(
                        frame,
                        area,
                        area_inner,
                        &par_of_overdue,
                        vertical_scroll_of_overdue,
                        &mut scrollbar_state_of_overdue,
                    );
                }
                View::Today => {
                    render_screen(
                        frame,
                        area,
                        area_inner,
                        &par_of_today,
                        vertical_scroll_of_today,
                        &mut scrollbar_state_of_today,
                    );
                }
                View::RestOfTheWeek => {
                    render_screen(
                        frame,
                        area,
                        area_inner,
                        &par_of_rest_of_the_week,
                        vertical_scroll_of_rest_of_the_week,
                        &mut scrollbar_state_of_rest_of_the_week,
                    );
                }
                View::LaterAndOther => {
                    render_screen(
                        frame,
                        area,
                        area_inner,
                        &par_of_later_and_other,
                        vertical_scroll_of_later_and_other,
                        &mut scrollbar_state_of_later_and_other,
                    );
                }
            }
        })?;

        if event::poll(EVENT_POLL_TIMEOUT)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('t') => {
                            current_view = View::Today;
                        }
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => match current_view {
                            View::Overdue => {
                                (vertical_scroll_of_overdue, scrollbar_state_of_overdue) =
                                    scroll_down(
                                        vertical_scroll_of_overdue,
                                        scrollbar_state_of_overdue,
                                    );
                            }
                            View::Today => {
                                (vertical_scroll_of_today, scrollbar_state_of_today) =
                                    scroll_down(vertical_scroll_of_today, scrollbar_state_of_today);
                            }
                            View::RestOfTheWeek => {
                                (
                                    vertical_scroll_of_rest_of_the_week,
                                    scrollbar_state_of_rest_of_the_week,
                                ) = scroll_down(
                                    vertical_scroll_of_rest_of_the_week,
                                    scrollbar_state_of_rest_of_the_week,
                                );
                            }
                            View::LaterAndOther => {
                                (
                                    vertical_scroll_of_later_and_other,
                                    scrollbar_state_of_later_and_other,
                                ) = scroll_down(
                                    vertical_scroll_of_later_and_other,
                                    scrollbar_state_of_later_and_other,
                                );
                            }
                        },
                        KeyCode::Char('k') | KeyCode::Up => match current_view {
                            View::Overdue => {
                                (vertical_scroll_of_overdue, scrollbar_state_of_overdue) =
                                    scroll_up(
                                        vertical_scroll_of_overdue,
                                        scrollbar_state_of_overdue,
                                    );
                            }
                            View::Today => {
                                (vertical_scroll_of_today, scrollbar_state_of_today) =
                                    scroll_up(vertical_scroll_of_today, scrollbar_state_of_today);
                            }
                            View::RestOfTheWeek => {
                                (
                                    vertical_scroll_of_rest_of_the_week,
                                    scrollbar_state_of_rest_of_the_week,
                                ) = scroll_up(
                                    vertical_scroll_of_rest_of_the_week,
                                    scrollbar_state_of_rest_of_the_week,
                                );
                            }
                            View::LaterAndOther => {
                                (
                                    vertical_scroll_of_later_and_other,
                                    scrollbar_state_of_later_and_other,
                                ) = scroll_up(
                                    vertical_scroll_of_later_and_other,
                                    scrollbar_state_of_later_and_other,
                                );
                            }
                        },
                        KeyCode::Char('h') | KeyCode::Left => {
                            current_view = current_view.prev();
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            current_view = current_view.next();
                        }
                        KeyCode::Char('1') => {
                            current_view = View::Overdue;
                        }
                        KeyCode::Char('2') => {
                            current_view = View::Today;
                        }
                        KeyCode::Char('3') => {
                            current_view = View::RestOfTheWeek;
                        }
                        KeyCode::Char('4') => {
                            current_view = View::LaterAndOther;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    return Ok(());
}

fn render_screen(
    frame: &mut Frame,
    area: Rect,
    area_inner: Rect,
    par_of_screen: &Paragraph,
    vertical_scroll: usize,
    scrollbar_state: &mut ScrollbarState,
) {
    frame.render_widget(
        par_of_screen.clone().scroll((vertical_scroll as u16, 0)),
        area,
    );
    frame.render_stateful_widget(create_scrollbar(), area_inner, scrollbar_state);
}

fn scroll_up(vertical_scroll: usize, scrollbar_state: ScrollbarState) -> (usize, ScrollbarState) {
    return (
        vertical_scroll.saturating_sub(1),
        scrollbar_state.position(vertical_scroll),
    );
}

fn scroll_down(vertical_scroll: usize, scrollbar_state: ScrollbarState) -> (usize, ScrollbarState) {
    return (
        vertical_scroll.saturating_add(1),
        scrollbar_state.position(vertical_scroll),
    );
}

fn create_scrollbar() -> Scrollbar<'static> {
    return Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
}
