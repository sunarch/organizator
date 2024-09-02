// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
// dependencies
use chrono::{Datelike, NaiveDate};
// internal
use crate::tasks::task::Task;
use crate::tasks::task_data::TaskData;
use crate::time;
use crate::words;

pub fn print_to_file(task_data_ref: &TaskData, data_dir_todo_output: PathBuf) {
    let output_file_name: &str = "dated.md";
    let output_file_path: PathBuf = data_dir_todo_output.join(output_file_name);
    println!(
        "Writing to output file '{}",
        output_file_path.clone().display()
    );
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

    print_list(task_data_ref, &mut Some(file));
}

pub fn print_to_console(task_data_ref: &TaskData) {
    print_list(task_data_ref, &mut None);
}

fn print_list(task_data: &TaskData, file_opt_ref: &mut Option<File>) {
    print_title(file_opt_ref);

    print_section_general(&task_data.sections.overdue, "", file_opt_ref);

    {
        let heading: String = format!(
            ">>>  TODAY  -  {} {}. ({}) <<<",
            time::month_abbrev(task_data.dates.today.month()),
            task_data.dates.today.day(),
            time::weekday_abbrev(&task_data.dates.today)
        );
        print_section_list(&task_data.sections.today, heading.as_str(), file_opt_ref);
    }

    print_section_heading(task_data.dates.today.year(), file_opt_ref);
    let mut dt_next: NaiveDate = task_data.dates.today;
    loop {
        dt_next = time::increment_day(&dt_next);

        if time::is_monday(&dt_next) {
            let dt_sunday: NaiveDate = time::monday_to_sunday(&dt_next);
            if time::is_day_in_first_week_of_year(&dt_sunday) {
                print_section_heading(dt_sunday.year(), file_opt_ref);
            }

            print_week_heading(&dt_next, file_opt_ref)
        }

        match task_data.sections.dated.get_key_value(&dt_next) {
            None => {}
            Some((_, task_list)) => {
                print_day_heading(&dt_next, file_opt_ref);
                print_task_list(task_list, file_opt_ref);
            }
        }

        if dt_next.eq(&task_data.dates.last_dated) {
            break;
        }
    }

    print_section_general(&task_data.sections.later, "later", file_opt_ref);

    print_section_list(&task_data.sections.inactive, "inactive", file_opt_ref);

    print_bottom_line(file_opt_ref);
}

fn print_title(file_opt_ref: &mut Option<File>) {
    let line: String = format!("# 🗓️ {}", words::DATED_TITLE);
    print_dual(&line, file_opt_ref);
}

fn print_bottom_line(file_opt_ref: &mut Option<File>) {
    print_empty_line(file_opt_ref);
    let line = String::from("---");
    print_dual(&line, file_opt_ref);
}

fn print_section_heading<T: Display>(text: T, file_opt_ref: &mut Option<File>) {
    print_empty_line(file_opt_ref);
    {
        let line = String::from("---");
        print_dual(&line, file_opt_ref);
    }
    {
        let line: String = format!("## {}", text);
        print_dual(&line, file_opt_ref);
    }
}

fn print_week_heading(date_ref: &NaiveDate, file_opt_ref: &mut Option<File>) {
    print_empty_line(file_opt_ref);
    print_dual(&time::week_timestamp(date_ref), file_opt_ref);
}

fn print_day_heading(date_ref: &NaiveDate, file_opt_ref: &mut Option<File>) {
    print_empty_line(file_opt_ref);
    print_dual(&time::day_timestamp(date_ref), file_opt_ref);
}

fn print_section_general(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    heading: &str,
    file_opt_ref: &mut Option<File>,
) {
    if task_map.is_empty() {
        return;
    }

    if !heading.is_empty() {
        print_section_heading(heading, file_opt_ref);
    }
    for (task_date, task_list) in task_map {
        print_day_heading(task_date, file_opt_ref);
        print_task_list(task_list, file_opt_ref);
    }
}

fn print_section_list(task_list: &Vec<Task>, heading: &str, file_opt_ref: &mut Option<File>) {
    if task_list.is_empty() {
        return;
    }
    print_section_heading(heading, file_opt_ref);
    print_empty_line(file_opt_ref);
    print_task_list(task_list, file_opt_ref);
}

fn print_task_list(task_list: &Vec<Task>, file_opt_ref: &mut Option<File>) {
    for task in task_list {
        if task.active {
            print_dual(&format!("- [ ] {}", task), file_opt_ref);
        } else {
            print_dual(&format!("- {}", task), file_opt_ref);
        }
        for subtask in &task.subtasks {
            if subtask.done.is_empty() {
                print_dual(&format!("    - [ ] {}", subtask.title), file_opt_ref);
            }
        }
    }
}

fn print_empty_line(file_opt_ref: &mut Option<File>) {
    print_dual(&String::from(""), file_opt_ref);
}

fn print_dual(line: &String, file_opt_ref: &mut Option<File>) {
    match file_opt_ref {
        None => {
            println!("{}", line);
        }
        Some(file_ref) => {
            if let Err(why) = file_ref.write_all(line.as_bytes()) {
                panic!("Couldn't write to output file\n{}", why);
            }
            if let Err(why) = file_ref.write_all(String::from("\n").as_bytes()) {
                panic!("Couldn't write to output file\n{}", why);
            }
            file_ref
                .flush()
                .expect("Unable to flush write to output file.");
        }
    }
}
