// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
// dependencies
use chrono::{NaiveDate, NaiveWeek};
// internal
use crate::tasks::data::TaskData;
use crate::tasks::task::contents::TaskVisibility;
use crate::tasks::task::Task;
use crate::time;
use crate::time::timestamp;
use crate::words;

pub(crate) type FnOutput = dyn Fn(&String, &mut Option<File>);

pub(crate) fn print_list(
    task_data: &TaskData,
    output_fn: &FnOutput,
    file_option: &mut Option<File>,
) {
    print_title(output_fn, file_option);

    // no heading for overdue section
    print_section_general(&task_data.sections.overdue, output_fn, file_option);

    print_part_today(
        &task_data.dates.today,
        &task_data.sections.today,
        output_fn,
        file_option,
    );

    print_section_heading(task_data.dates.current_year, output_fn, file_option);
    print_section_general(&task_data.sections.rest_of_the_week, output_fn, file_option);
    print_section_dated(
        &task_data.sections.dated,
        &task_data.dates.dated_weeks_current_year,
        output_fn,
        file_option,
    );

    print_section_heading(task_data.dates.next_year, output_fn, file_option);
    print_section_dated(
        &task_data.sections.dated,
        &task_data.dates.dated_weeks_next_year,
        output_fn,
        file_option,
    );

    print_section_heading(words::LATER, output_fn, file_option);
    print_section_general(&task_data.sections.later, output_fn, file_option);

    print_section_heading(words::INACTIVE, output_fn, file_option);
    print_section_list(&task_data.sections.inactive, output_fn, file_option);

    print_bottom_line(output_fn, file_option);
}

pub(crate) fn print_part_today(
    today: &NaiveDate,
    task_list: &Vec<Task>,
    output_fn: &FnOutput,
    file_option: &mut Option<File>,
) {
    {
        let heading: String = format!(
            ">>>  {}  -  {} <<<",
            words::TODAY.to_uppercase(),
            timestamp::day_short(today)
        );
        print_section_heading(heading.as_str(), output_fn, file_option);
    }
    print_section_list(task_list, output_fn, file_option);
}

fn print_title(output_fn: &FnOutput, file_option: &mut Option<File>) {
    #[allow(clippy::const_is_empty)]
    let icon_spacing: &str = if words::DATED_TITLE_ICON.is_empty() {
        ""
    } else {
        " "
    };
    let line: String = format!(
        "# {}️{}️{}",
        words::DATED_TITLE_ICON,
        icon_spacing,
        words::DATED_TITLE
    );
    output_fn(&line, file_option);
}

fn print_bottom_line(output_fn: &FnOutput, file_option: &mut Option<File>) {
    print_empty_line(output_fn, file_option);
    let line = String::from("---");
    output_fn(&line, file_option);
}

fn print_section_heading<T: Display>(
    text: T,
    output_fn: &FnOutput,
    file_option: &mut Option<File>,
) {
    print_empty_line(output_fn, file_option);
    {
        let line = String::from("---");
        output_fn(&line, file_option);
    }
    {
        let line: String = format!("## {}", text);
        output_fn(&line, file_option);
    }
}

fn print_week_heading(date: &NaiveDate, output_fn: &FnOutput, file_option: &mut Option<File>) {
    print_empty_line(output_fn, file_option);
    output_fn(
        &format!("#### {}", timestamp::week(date, false)),
        file_option,
    );
}

fn print_day_heading(date: &NaiveDate, output_fn: &FnOutput, file_option: &mut Option<File>) {
    print_empty_line(output_fn, file_option);
    output_fn(&timestamp::day(date), file_option);
}

fn print_section_general(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    output_fn: &FnOutput,
    file_option: &mut Option<File>,
) {
    if task_map.is_empty() {
        return;
    }

    for (task_date, task_list) in task_map {
        print_day_heading(task_date, output_fn, file_option);
        print_task_list(task_list, output_fn, file_option);
    }
}

fn print_section_list(task_list: &Vec<Task>, output_fn: &FnOutput, file_option: &mut Option<File>) {
    if task_list.is_empty() {
        return;
    }
    print_empty_line(output_fn, file_option);
    print_task_list(task_list, output_fn, file_option);
}

fn print_section_dated(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    week_list: &Vec<NaiveWeek>,
    output_fn: &FnOutput,
    file_option: &mut Option<File>,
) {
    for week in week_list {
        print_week_heading(&week.first_day(), output_fn, file_option);

        for day in time::iterate_week(week) {
            if let Some((_, task_list)) = task_map.get_key_value(&day) {
                print_day_heading(&day, output_fn, file_option);
                print_task_list(task_list, output_fn, file_option);
            }
        }
    }
}

fn print_task_list(task_list: &Vec<Task>, output_fn: &FnOutput, file_option: &mut Option<File>) {
    for task in task_list {
        print_task(task, output_fn, file_option);
    }
}

fn print_task(task: &Task, output_fn: &FnOutput, file_option: &mut Option<File>) {
    let done_marker: &str = if task.contents.is_done { "x" } else { " " };

    match task.contents.visibility {
        TaskVisibility::Visible => {
            output_fn(&format!("- [{}] {}", done_marker, task), file_option);
        }
        TaskVisibility::Inactive => output_fn(&format!("- {}", task), file_option),
        TaskVisibility::Hidden => return,
    }

    for subtask in &task.meta.subtasks {
        let done_marker: &str = if subtask.is_done { "x" } else { " " };
        let note: String = if subtask.note.is_empty() {
            Default::default()
        } else {
            format!(" ({})", subtask.note)
        };

        match subtask.visibility {
            TaskVisibility::Visible => output_fn(
                &format!("    - [{}] {}{}", done_marker, subtask.title, note),
                file_option,
            ),
            TaskVisibility::Inactive => output_fn(
                &format!("    - ~~[{}] {}{}~~", done_marker, subtask.title, note),
                file_option,
            ),
            TaskVisibility::Hidden => continue,
        }
    }
}

fn print_empty_line(output_fn: &FnOutput, file_option: &mut Option<File>) {
    output_fn(&String::from(""), file_option);
}
