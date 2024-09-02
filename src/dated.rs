// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
// dependencies
use chrono::{Datelike, NaiveDate, Weekday};
// internal
use crate::tasks::task::Task;
use crate::tasks::task_sections::TaskSections;
use crate::time;
use crate::words;

pub fn print_list(task_sections: &TaskSections, data_dir_todo_output: PathBuf) {
    let output_file_name: &str = "dated.md";
    let output_file_path: PathBuf = data_dir_todo_output.join(output_file_name);
    println!(
        "Writing to output file '{}",
        output_file_path.clone().display()
    );
    let mut file = match File::create(output_file_path.clone()) {
        Err(why) => {
            panic!(
                "Couldn't open output file  '{}'\n{}",
                output_file_path.clone().display(),
                why
            );
        }
        Ok(file) => file,
    };
    let file_ref: &mut File = &mut file;

    let (today, last_dated) = time::today_and_last_dated();

    print_title(file_ref);

    print_section_general(&task_sections.overdue, "", file_ref);

    {
        let heading: String = format!(
            ">>>  TODAY  -  {} {}. ({}) <<<",
            time::month_abbrev(today.month()),
            today.day(),
            time::weekday_abbrev(&today)
        );
        print_section_list(&task_sections.today, heading.as_str(), file_ref);
    }

    print_section_heading(today.year(), file_ref);
    let mut dt_next: NaiveDate = today;
    loop {
        dt_next = dt_next
            .checked_add_days(time::DAYS_1)
            .expect("Failed adding day");

        if dt_next.weekday() == Weekday::Mon {
            let dt_sunday: NaiveDate = dt_next
                .checked_add_days(time::DAYS_6)
                .expect("Failed adding days");
            if format!("{:?}", dt_sunday.iso_week()).ends_with("01") {
                print_section_heading(dt_sunday.year(), file_ref);
            }

            print_week_heading(&dt_next, &dt_sunday, file_ref)
        }

        let date_for_tasks =
            NaiveDate::from_ymd_opt(dt_next.year(), dt_next.month(), dt_next.day())
                .expect("Failed to create NaiveDate");
        match task_sections.dated.get_key_value(&date_for_tasks) {
            None => {}
            Some((_, task_list)) => {
                print_day_heading(&date_for_tasks, file_ref);
                print_task_list(task_list, file_ref);
            }
        }

        if dt_next.eq(&last_dated) {
            break;
        }
    }

    print_section_general(&task_sections.later, "later", file_ref);

    print_section_list(&task_sections.inactive, "inactive", file_ref);

    print_bottom_line(file_ref);
}

fn print_title(file_ref: &mut File) {
    let line: String = format!("# 🗓️ {}", words::DATED_TITLE);
    print_dual(&line, file_ref);
}

fn print_bottom_line(file_ref: &mut File) {
    print_empty_line(file_ref);
    let line = String::from("---");
    print_dual(&line, file_ref);
}

fn print_section_heading<T: Display>(text: T, file_ref: &mut File) {
    print_empty_line(file_ref);
    {
        let line = String::from("---");
        print_dual(&line, file_ref);
    }
    {
        let line: String = format!("## {}", text);
        print_dual(&line, file_ref);
    }
}

fn print_week_heading(dt_next: &NaiveDate, dt_sunday: &NaiveDate, file_ref: &mut File) {
    let mut date_range_display: String =
        format!("{} {}", time::month_abbrev(dt_next.month()), dt_next.day());

    if dt_next.month() == dt_sunday.month() {
        date_range_display = format!("{}-{}.", date_range_display, dt_sunday.day());
    } else {
        date_range_display = format!(
            "{}. - {} {}.",
            date_range_display,
            time::month_abbrev(dt_sunday.month()),
            dt_sunday.day()
        );
    }

    print_empty_line(file_ref);
    {
        let line: String = format!("#### {:?} ({})", dt_next.iso_week(), date_range_display);
        print_dual(&line, file_ref);
    }
}

fn print_day_heading(date_ref: &NaiveDate, file_ref: &mut File) {
    let line: String = format!(
        "{}-{:0>2}-{:0>2} ({})",
        date_ref.year(),
        date_ref.month(),
        date_ref.day(),
        time::weekday_abbrev(date_ref)
    );

    print_empty_line(file_ref);
    print_dual(&line, file_ref);
}

fn print_section_general(
    task_map: &BTreeMap<NaiveDate, Vec<Task>>,
    heading: &str,
    file_ref: &mut File,
) {
    if task_map.is_empty() {
        return;
    }

    if !heading.is_empty() {
        print_section_heading(heading, file_ref);
    }
    for (task_date, task_list) in task_map {
        print_day_heading(task_date, file_ref);
        print_task_list(task_list, file_ref);
    }
}

fn print_section_list(task_list: &Vec<Task>, heading: &str, file_ref: &mut File) {
    if task_list.is_empty() {
        return;
    }
    print_section_heading(heading, file_ref);
    print_empty_line(file_ref);
    print_task_list(task_list, file_ref);
}

fn print_task_list(task_list: &Vec<Task>, file_ref: &mut File) {
    for task in task_list {
        if task.active {
            print_dual(&format!("- [ ] {}", task), file_ref);
        } else {
            print_dual(&format!("- {}", task), file_ref);
        }
        for subtask in &task.subtasks {
            if subtask.done.is_empty() {
                print_dual(&format!("    - [ ] {}", subtask.title), file_ref);
            }
        }
    }
}

fn print_empty_line(file_ref: &mut File) {
    print_dual(&String::from(""), file_ref);
}

fn print_dual(line: &String, file_ref: &mut File) {
    println!("{}", line);
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
