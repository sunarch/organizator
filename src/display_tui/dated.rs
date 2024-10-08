// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fmt::Display;
// dependencies
use chrono::{NaiveDate, NaiveWeek};
use ratatui::layout::Alignment;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{block::Title, Block, Paragraph, ScrollbarState, Wrap};
// internal
use crate::tasks::data::TaskData;
use crate::tasks::task::contents::TaskVisibility;
use crate::tasks::task::Task;
use crate::time;
use crate::time::timestamp;
use crate::words;

const INITIAL_SCROLL: usize = 0;

pub(super) struct DatedView {
    pub(super) content_length: usize,
    pub(super) vertical_scroll: usize,
    pub(super) scrollbar_state: ScrollbarState,
}

impl Default for DatedView {
    fn default() -> Self {
        return Self {
            content_length: Default::default(),
            vertical_scroll: Default::default(),
            scrollbar_state: Default::default(),
        };
    }
}

impl DatedView {
    pub(super) fn new(content_length: usize) -> Self {
        return Self {
            content_length,
            vertical_scroll: INITIAL_SCROLL,
            scrollbar_state: ScrollbarState::new(content_length).position(INITIAL_SCROLL),
        };
    }
}

fn par<'a>(lines: Vec<Line<'a>>, title: &'static str) -> (Paragraph<'a>, DatedView) {
    let line_count: usize = lines.len();
    const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
    return (
        Paragraph::new(lines)
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((1, 0))
            .block(
                Block::bordered()
                    .title(par_create_title(PROGRAM_NAME, Alignment::Left, false))
                    .title(par_create_title(title, Alignment::Center, true))
                    .title(par_create_title(words::QUIT_NOTE, Alignment::Right, false)),
            ),
        DatedView::new(line_count),
    );
}

fn par_create_title(text: &str, alignment: Alignment, to_bold: bool) -> Title {
    let mut line: Line = Line::from(format!("[ {} ]", text));
    if to_bold {
        line = line.bold();
    }
    return Title::from(line).alignment(alignment);
}

pub(super) fn par_of_overdue(task_data: &TaskData) -> (Paragraph, DatedView) {
    let mut lines: Vec<Line> = Default::default();

    // no heading for overdue section
    add_section_general(&task_data.sections.overdue, &mut lines);

    return par(lines, words::TITLE_OVERDUE);
}

pub(super) fn par_of_today(task_data: &TaskData) -> (Paragraph, DatedView) {
    let mut lines: Vec<Line> = Default::default();

    {
        let heading: String = format!(
            ">>>  {}  -  {} <<<",
            words::TODAY.to_uppercase(),
            timestamp::day_short(&task_data.dates.today)
        );
        add_section_heading(heading.as_str(), &mut lines);
    }
    add_section_list(&task_data.sections.today, &mut lines);

    if task_data.sections.today.is_empty() {
        add_empty_line(&mut lines);
        add_empty_line(&mut lines);
        add_empty_line(&mut lines);
        lines.push(
            Line::from(vec![Span::styled(
                words::NOTE_DONE_FOR_TODAY,
                Modifier::BOLD,
            )])
            .centered(),
        );
    }

    return par(lines, words::TITLE_TODAY);
}

pub(super) fn par_of_rest_of_the_week(task_data: &TaskData) -> (Paragraph, DatedView) {
    let mut lines: Vec<Line> = Default::default();

    add_section_general(&task_data.sections.rest_of_the_week, &mut lines);

    return par(lines, words::TITLE_REST_OF_THE_WEEK);
}

pub(super) fn par_of_later_and_other(task_data: &TaskData) -> (Paragraph, DatedView) {
    let mut lines: Vec<Line> = Default::default();

    add_section_heading(task_data.dates.current_year, &mut lines);
    add_section_dated(
        &task_data.sections.dated,
        &task_data.dates.dated_weeks_current_year,
        &mut lines,
    );

    add_section_heading(task_data.dates.next_year, &mut lines);
    add_section_dated(
        &task_data.sections.dated,
        &task_data.dates.dated_weeks_next_year,
        &mut lines,
    );

    add_section_heading(words::LATER, &mut lines);
    add_section_general(&task_data.sections.later, &mut lines);

    add_section_heading(words::INACTIVE, &mut lines);
    add_section_list(&task_data.sections.inactive, &mut lines);

    return par(lines, words::TITLE_LATER_AND_OTHER);
}

fn add_section_heading<T: Display>(text: T, lines: &mut Vec<Line>) {
    add_empty_line(lines);

    const FILL_CHAR: &str = "=";
    const SIDE_WIDTH: usize = 3;
    let side: String = FILL_CHAR.repeat(SIDE_WIDTH);
    let content_line: String = format!("{} {} {}", side, text, side);
    let top_and_bottom: String = FILL_CHAR.repeat(content_line.len());

    lines.push(Line::from(vec![Span::styled(top_and_bottom.clone(), Modifier::BOLD)]).centered());
    lines.push(Line::from(vec![Span::styled(content_line, Modifier::BOLD)]).centered());
    lines.push(Line::from(vec![Span::styled(top_and_bottom, Modifier::BOLD)]).centered());
}

fn add_week_heading(date: &NaiveDate, lines: &mut Vec<Line>) {
    add_empty_line(lines);
    lines.push(Line::from(format!("{} ", timestamp::week(date, false))).right_aligned());
}

fn add_day_heading(date: &NaiveDate, lines: &mut Vec<Line>) {
    add_empty_line(lines);
    lines.push(Line::from(vec![Span::styled(
        timestamp::day(date),
        Modifier::BOLD,
    )]));
}

fn add_section_general(task_map: &BTreeMap<NaiveDate, Vec<Task>>, lines: &mut Vec<Line>) {
    for (task_date, task_list) in task_map {
        add_day_heading(task_date, lines);
        add_task_list(task_list, lines);
    }
}

fn add_section_list(task_list: &Vec<Task>, lines: &mut Vec<Line>) {
    if task_list.is_empty() {
        return;
    }
    add_task_list(task_list, lines);
}

fn add_section_dated(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    week_list: &Vec<NaiveWeek>,
    lines: &mut Vec<Line>,
) {
    for week in week_list {
        add_week_heading(&week.first_day(), lines);

        for day in time::iterate_week(week) {
            if let Some((_, task_list)) = task_map.get_key_value(&day) {
                add_day_heading(&day, lines);
                add_task_list(task_list, lines);
            }
        }
    }
}

fn add_task_list(task_list: &Vec<Task>, lines: &mut Vec<Line>) {
    add_empty_line(lines);
    for task in task_list {
        add_task(task, lines);
    }
}

fn add_task(task: &Task, lines: &mut Vec<Line>) {
    let done_marker: &str = if task.contents.is_done { "x" } else { " " };

    match task.contents.visibility {
        TaskVisibility::Visible => lines.push(Line::from(format!(
            "- [{}] {} {}",
            done_marker,
            task.meta.format_as_table_row(),
            task.contents
        ))),
        TaskVisibility::Inactive => lines.push(Line::from(format!(
            "- {} {}",
            task.meta.format_as_table_row(),
            task.contents
        ))),
        TaskVisibility::Hidden => {
            return Default::default();
        }
    }

    const SUBTASK_INDENT: usize = 24; // manual from meta width
    for subtask in &task.meta.subtasks {
        let done_marker: &str = if subtask.is_done { "x" } else { " " };
        let note: String = if subtask.note.is_empty() {
            Default::default()
        } else {
            format!(" ({})", subtask.note)
        };

        match subtask.visibility {
            TaskVisibility::Visible => lines.push(Line::from(format!(
                "{}- [{}] {}{}",
                " ".repeat(SUBTASK_INDENT),
                done_marker,
                subtask.title,
                note
            ))),
            TaskVisibility::Inactive => lines.push(Line::from(format!(
                "{}- ~~[{}] {}{}~~",
                " ".repeat(SUBTASK_INDENT),
                done_marker,
                subtask.title,
                note
            ))),
            TaskVisibility::Hidden => continue,
        }
    }
}

fn add_empty_line(lines: &mut Vec<Line>) {
    lines.push(Line::from(""));
}
