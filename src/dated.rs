// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
// dependencies
use chrono::{Datelike, NaiveDate, NaiveWeek};
// internal
use crate::logging;
use crate::tasks::task::Task;
use crate::tasks::task_data::TaskData;
use crate::time;
use crate::words;

pub fn print_to_file(task_data: &TaskData, data_dir_todo_output: PathBuf) {
    let output_file_name: &str = "dated.md";
    let output_file_path: PathBuf = data_dir_todo_output.join(output_file_name);
    logging::info(format!(
        "Writing to output file '{}",
        output_file_path.clone().display()
    ));
    let file: File = match File::create(output_file_path.clone()) {
        Err(why) => {
            panic!(
                "Couldn't open output file  '{}'\n{}",
                output_file_path.clone().display(),
                why
            );
        }
        Ok(file) => file,
    };

    print_list(task_data, &mut Some(file));
}

pub fn print_to_console(task_data: &TaskData) {
    print_list(task_data, &mut None);
}

fn print_list(task_data: &TaskData, file_option: &mut Option<File>) {
    print_title(file_option);

    // no heading for overdue section
    print_section_general(&task_data.sections.overdue, file_option);

    {
        let heading: String = format!(
            ">>>  TODAY  -  {} {}. ({}) <<<",
            time::month_abbrev(task_data.dates.today.month()),
            task_data.dates.today.day(),
            time::weekday_abbrev(&task_data.dates.today)
        );
        print_section_heading(heading.as_str(), file_option);
    }
    print_section_list(&task_data.sections.today, file_option);

    print_section_heading(task_data.dates.current_year, file_option);
    print_section_general(&task_data.sections.dated_current_week, file_option);
    print_section_dated(
        &task_data.sections.dated,
        &task_data.dates.dated_weeks_current_year,
        file_option,
    );

    print_section_heading(task_data.dates.next_year, file_option);
    print_section_dated(
        &task_data.sections.dated,
        &task_data.dates.dated_weeks_next_year,
        file_option,
    );

    print_section_heading("later", file_option);
    print_section_general(&task_data.sections.later, file_option);

    print_section_heading("inactive", file_option);
    print_section_list(&task_data.sections.inactive, file_option);

    print_bottom_line(file_option);
}

fn print_title(file_option: &mut Option<File>) {
    let line: String = format!("# 🗓️ {}", words::DATED_TITLE);
    print_dual(&line, file_option);
}

fn print_bottom_line(file_option: &mut Option<File>) {
    print_empty_line(file_option);
    let line = String::from("---");
    print_dual(&line, file_option);
}

fn print_section_heading<T: Display>(text: T, file_option: &mut Option<File>) {
    print_empty_line(file_option);
    {
        let line = String::from("---");
        print_dual(&line, file_option);
    }
    {
        let line: String = format!("## {}", text);
        print_dual(&line, file_option);
    }
}

fn print_week_heading(date: &NaiveDate, file_option: &mut Option<File>) {
    print_empty_line(file_option);
    print_dual(&time::week_timestamp(date), file_option);
}

fn print_day_heading(date: &NaiveDate, file_option: &mut Option<File>) {
    print_empty_line(file_option);
    print_dual(&time::day_timestamp(date), file_option);
}

fn print_section_general(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    file_option: &mut Option<File>,
) {
    if task_map.is_empty() {
        return;
    }

    for (task_date, task_list) in task_map {
        print_day_heading(task_date, file_option);
        print_task_list(task_list, file_option);
    }
}

fn print_section_list(task_list: &Vec<Task>, file_option: &mut Option<File>) {
    if task_list.is_empty() {
        return;
    }
    print_empty_line(file_option);
    print_task_list(task_list, file_option);
}

fn print_section_dated(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    week_list: &Vec<NaiveWeek>,
    file_option: &mut Option<File>,
) {
    for week in week_list {
        print_week_heading(&week.first_day(), file_option);

        for day in time::iterate_week(week) {
            if let Some((_, task_list)) = task_map.get_key_value(&day) {
                print_day_heading(&day, file_option);
                print_task_list(task_list, file_option);
            }
        }
    }
}

fn print_task_list(task_list: &Vec<Task>, file_option: &mut Option<File>) {
    for task in task_list {
        if task.active {
            print_dual(&format!("- [ ] {}", task), file_option);
        } else {
            print_dual(&format!("- {}", task), file_option);
        }
        for subtask in &task.subtasks {
            if subtask.done.is_empty() {
                print_dual(&format!("    - [ ] {}", subtask.title), file_option);
            }
        }
    }
}

fn print_empty_line(file_option: &mut Option<File>) {
    print_dual(&String::from(""), file_option);
}

fn print_dual(line: &String, file_option: &mut Option<File>) {
    match file_option {
        None => {
            println!("{}", line);
        }
        Some(file) => {
            if let Err(why) = file.write_all(line.as_bytes()) {
                panic!("Couldn't write to output file\n{}", why);
            }
            if let Err(why) = file.write_all(String::from("\n").as_bytes()) {
                panic!("Couldn't write to output file\n{}", why);
            }
            file.flush().expect("Unable to flush write to output file.");
        }
    }
}
