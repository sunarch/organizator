// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod dated;
mod tui_current_view;

use std::collections::HashMap;
use std::io;
use std::time::Duration;
// dependencies
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Margin, Rect};
use ratatui::widgets::{
    Paragraph, ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use ratatui::DefaultTerminal;
use ratatui::Frame;
// internal
use crate::display_tui::dated::DatedView;
use crate::display_tui::tui_current_view::CurrentView;
use crate::logging;
use crate::tasks::data::TaskData;

const EVENT_POLL_TIMEOUT: Duration = Duration::from_millis(16);
const DEFAULT_SCROLL_AMOUNT: usize = 1;
const DEFAULT_SCROLL_PG_OVERLAP: usize = DEFAULT_SCROLL_AMOUNT + 3;

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
    current_height: u16,

    view_overdue: DatedView,
    view_today: DatedView,
    view_rest_of_the_week: DatedView,
    view_later_and_other: DatedView,
}

impl Tui {
    fn new() -> Self {
        return Self {
            current_view: Default::default(),
            current_height: 1,

            view_overdue: Default::default(),
            view_today: Default::default(),
            view_rest_of_the_week: Default::default(),
            view_later_and_other: Default::default(),
        };
    }

    fn current_view_set(&mut self, new_current_view: CurrentView) {
        self.current_view = new_current_view;
    }

    fn current_view_prev(&mut self) {
        self.current_view = self.current_view.prev();
    }

    fn current_view_next(&mut self) {
        self.current_view = self.current_view.next();
    }

    fn get_view(&mut self) -> &mut DatedView {
        return match self.current_view {
            CurrentView::Overdue => &mut self.view_overdue,
            CurrentView::Today => &mut self.view_today,
            CurrentView::RestOfTheWeek => &mut self.view_rest_of_the_week,
            CurrentView::LaterAndOther => &mut self.view_later_and_other,
        };
    }

    fn scroll(&mut self, direction: ScrollDirection, amount: usize) {
        let view: &mut DatedView = self.get_view();
        match direction {
            ScrollDirection::Forward => {
                view.vertical_scroll = view
                    .content_length
                    .min(view.vertical_scroll.saturating_add(amount));
            }
            ScrollDirection::Backward => {
                view.vertical_scroll = view.vertical_scroll.saturating_sub(amount);
            }
        }
        view.scrollbar_state = view.scrollbar_state.position(view.vertical_scroll);
    }

    fn scroll_down(&mut self) {
        self.scroll(ScrollDirection::Forward, DEFAULT_SCROLL_AMOUNT);
    }

    fn scroll_up(&mut self) {
        self.scroll(ScrollDirection::Backward, DEFAULT_SCROLL_AMOUNT);
    }

    fn scroll_pg_down(&mut self) {
        self.scroll(
            ScrollDirection::Forward,
            (self.current_height as usize).saturating_sub(DEFAULT_SCROLL_PG_OVERLAP),
        );
    }

    fn scroll_pg_up(&mut self) {
        self.scroll(
            ScrollDirection::Backward,
            (self.current_height as usize).saturating_sub(DEFAULT_SCROLL_PG_OVERLAP),
        );
    }

    fn scroll_end(&mut self, direction: ScrollDirection) {
        let view: &mut DatedView = self.get_view();
        match direction {
            ScrollDirection::Forward => {
                view.vertical_scroll = view.content_length.saturating_sub(1);
            }
            ScrollDirection::Backward => {
                view.vertical_scroll = 0;
            }
        }
        view.scrollbar_state = view.scrollbar_state.position(view.vertical_scroll);
    }

    fn scroll_bottom(&mut self) {
        self.scroll_end(ScrollDirection::Forward);
    }

    fn scroll_top(&mut self) {
        self.scroll_end(ScrollDirection::Backward);
    }

    fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        task_data: &TaskData,
    ) -> Result<(), io::Error> {
        let par_of_overdue: Paragraph;
        let par_of_today: Paragraph;
        let par_of_rest_of_the_week: Paragraph;
        let par_of_later_and_other: Paragraph;

        (par_of_overdue, self.view_overdue) = dated::par_of_overdue(task_data);
        (par_of_today, self.view_today) = dated::par_of_today(task_data);
        (par_of_rest_of_the_week, self.view_rest_of_the_week) =
            dated::par_of_rest_of_the_week(task_data);
        (par_of_later_and_other, self.view_later_and_other) =
            dated::par_of_later_and_other(task_data);

        let mut par_map: HashMap<CurrentView, &Paragraph> = Default::default();
        par_map.insert(CurrentView::Overdue, &par_of_overdue);
        par_map.insert(CurrentView::Today, &par_of_today);
        par_map.insert(CurrentView::RestOfTheWeek, &par_of_rest_of_the_week);
        par_map.insert(CurrentView::LaterAndOther, &par_of_later_and_other);

        terminal.draw(|frame: &mut Frame| self.draw(frame, &par_map))?;

        loop {
            if event::poll(EVENT_POLL_TIMEOUT)? {
                match event::read()? {
                    Event::Key(key_event) => {
                        let to_quit: bool = self.handle_key_event(key_event);
                        if to_quit {
                            break;
                        }
                        terminal.draw(|frame: &mut Frame| self.draw(frame, &par_map))?;
                    }
                    Event::Resize(_, _) => {
                        terminal.draw(|frame: &mut Frame| self.draw(frame, &par_map))?;
                    }
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => return true,

                KeyCode::Char('h') | KeyCode::Left => self.current_view_prev(),
                KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                KeyCode::Char('l') | KeyCode::Right => self.current_view_next(),

                KeyCode::End => self.scroll_bottom(),
                KeyCode::Home => self.scroll_top(),

                KeyCode::PageDown => self.scroll_pg_down(),
                KeyCode::PageUp => self.scroll_pg_up(),

                KeyCode::Char('1') => self.current_view_set(CurrentView::Overdue),
                KeyCode::Char('2') => self.current_view_set(CurrentView::Today),
                KeyCode::Char('3') => self.current_view_set(CurrentView::RestOfTheWeek),
                KeyCode::Char('4') => self.current_view_set(CurrentView::LaterAndOther),

                KeyCode::Char('t') => self.current_view_set(CurrentView::Today),

                _ => {}
            }
        }

        return false;
    }

    fn draw(&mut self, frame: &mut Frame, par_map: &HashMap<CurrentView, &Paragraph>) {
        let area: Rect = frame.area();
        self.current_height = area.height;

        self.render_paragraph(frame, area, par_map);
        self.render_scrollbar(frame, area);
    }

    fn render_paragraph(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        par_map: &HashMap<CurrentView, &Paragraph>,
    ) {
        let paragraph: &Paragraph = par_map
            .get(&self.current_view)
            .expect("Unable to get Paragraph from map of references");
        let vertical_scroll: usize = match self.current_view {
            CurrentView::Overdue => self.view_overdue.vertical_scroll,
            CurrentView::Today => self.view_today.vertical_scroll,
            CurrentView::RestOfTheWeek => self.view_rest_of_the_week.vertical_scroll,
            CurrentView::LaterAndOther => self.view_later_and_other.vertical_scroll,
        };
        frame.render_widget(paragraph.clone().scroll((vertical_scroll as u16, 0)), area);
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
