// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod dated;

use std::io;
use std::time::Duration;
// dependencies
use ratatui::layout::Margin;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    Frame, Terminal,
};
// internal
use crate::logging;
use crate::tasks::data::TaskData;

enum View {
    Dated,
    Today,
}

impl Default for View {
    fn default() -> Self {
        return View::Today;
    }
}

pub(crate) fn run(task_data: &TaskData) -> io::Result<()> {
    logging::info("Running TUI ...".to_string());

    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let mut current_view: View = Default::default();

    let (par_of_dated, len_of_dated) = dated::par_of_all_dated(task_data);
    let mut vertical_scroll_of_dated: usize = 0;
    let mut scrollbar_state_of_dated: ScrollbarState =
        ScrollbarState::new(len_of_dated).position(vertical_scroll_of_dated);

    let (par_of_today, len_of_today) = dated::par_of_today(task_data);
    let mut vertical_scroll_of_today: usize = 0;
    let mut scrollbar_state_of_today: ScrollbarState =
        ScrollbarState::new(len_of_today).position(vertical_scroll_of_today);

    loop {
        terminal.draw(|frame: &mut Frame| {
            let area: Rect = frame.area();
            let area_inner: Rect = area.inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            });
            match current_view {
                View::Dated => {
                    frame.render_widget(
                        par_of_dated
                            .clone()
                            .scroll((vertical_scroll_of_dated as u16, 0)),
                        area,
                    );
                    frame.render_stateful_widget(
                        create_scrollbar(),
                        area_inner,
                        &mut scrollbar_state_of_dated,
                    );
                }
                View::Today => {
                    frame.render_widget(
                        par_of_today
                            .clone()
                            .scroll((vertical_scroll_of_today as u16, 0)),
                        area,
                    );
                    frame.render_stateful_widget(
                        create_scrollbar(),
                        area_inner,
                        &mut scrollbar_state_of_today,
                    );
                }
            }
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('d') => {
                            current_view = View::Dated;
                        }
                        KeyCode::Char('t') => {
                            current_view = View::Today;
                        }
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => match current_view {
                            View::Dated => {
                                vertical_scroll_of_dated =
                                    vertical_scroll_of_dated.saturating_add(1);
                                scrollbar_state_of_dated =
                                    scrollbar_state_of_dated.position(vertical_scroll_of_dated);
                            }
                            View::Today => {
                                vertical_scroll_of_today =
                                    vertical_scroll_of_today.saturating_add(1);
                                scrollbar_state_of_today =
                                    scrollbar_state_of_today.position(vertical_scroll_of_today);
                            }
                        },
                        KeyCode::Char('k') | KeyCode::Up => match current_view {
                            View::Dated => {
                                vertical_scroll_of_dated =
                                    vertical_scroll_of_dated.saturating_sub(1);
                                scrollbar_state_of_dated =
                                    scrollbar_state_of_dated.position(vertical_scroll_of_dated);
                            }
                            View::Today => {
                                vertical_scroll_of_today =
                                    vertical_scroll_of_today.saturating_sub(1);
                                scrollbar_state_of_today =
                                    scrollbar_state_of_today.position(vertical_scroll_of_today);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    logging::info("Exiting TUI ...".to_string());

    return Ok(());
}

fn create_scrollbar() -> Scrollbar<'static> {
    return Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
}
