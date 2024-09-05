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
use ratatui::widgets::{Paragraph, Wrap};
// internal
use crate::tasks::data::TaskData;
use crate::tasks::task::contents::TaskVisibility;
use crate::tasks::task::Task;
use crate::time::timestamp;
use crate::{time, words};

fn par(lines: Vec<Line>) -> (Paragraph, usize) {
    let line_count: usize = lines.len();
    return (
        Paragraph::new(lines)
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((1, 0)),
        line_count,
    );
}

pub(crate) fn par_of_dated(task_data: &TaskData) -> (Paragraph, usize) {
    let mut lines: Vec<Line> = Default::default();

    add_title(&mut lines);

    // no heading for overdue section
    add_section_general(&task_data.sections.overdue, &mut lines);

    part_today(
        &task_data.dates.today,
        &task_data.sections.today,
        &mut lines,
    );

    add_section_heading(task_data.dates.current_year, &mut lines);
    add_section_general(&task_data.sections.dated_current_week, &mut lines);
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

    return par(lines);
}

pub(crate) fn par_of_today(task_data: &TaskData) -> (Paragraph, usize) {
    let mut lines: Vec<Line> = Default::default();

    part_today(
        &task_data.dates.today,
        &task_data.sections.today,
        &mut lines,
    );

    return par(lines);
}

pub(crate) fn part_today(today: &NaiveDate, task_list: &Vec<Task>, lines: &mut Vec<Line>) {
    {
        let heading: String = format!(
            ">>>  {}  -  {} <<<",
            words::TODAY.to_uppercase(),
            timestamp::day_short(today)
        );
        add_section_heading(heading.as_str(), lines);
    }
    add_section_list(task_list, lines);
}

fn add_title(lines: &mut Vec<Line>) {
    #[allow(clippy::const_is_empty)]
    let icon_spacing: &str = if words::DATED_TITLE_ICON.is_empty() {
        ""
    } else {
        " "
    };

    add_empty_line(lines);
    lines.push(
        Line::from(vec![
            Span::raw(words::DATED_TITLE_ICON),
            Span::raw(icon_spacing),
            Span::styled(words::DATED_TITLE, Modifier::BOLD),
        ])
        .centered(),
    );
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
        TaskVisibility::Visible => lines.push(Line::from(format!("- [{}] {}", done_marker, task))),
        TaskVisibility::Inactive => lines.push(Line::from(format!("- {}", task))),
        TaskVisibility::Hidden => {
            return Default::default();
        }
    }

    for subtask in &task.meta.subtasks {
        let done_marker: &str = if subtask.is_done { "x" } else { " " };

        match subtask.visibility {
            TaskVisibility::Visible => lines.push(Line::from(format!(
                "    - [{}] {}",
                done_marker, subtask.title
            ))),
            TaskVisibility::Inactive => lines.push(Line::from(format!(
                "    - ~~[{}] {}~~",
                done_marker, subtask.title
            ))),
            TaskVisibility::Hidden => continue,
        }
    }
}

fn add_empty_line(lines: &mut Vec<Line>) {
    lines.push(Line::from(""));
}
