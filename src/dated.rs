// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use chrono::{Datelike, DateTime, Local, NaiveDate, Months, Days, Weekday};

use crate::tasks::TaskList;
use crate::words;

const MONTHS_12: Months = Months::new(12);
const DAYS_1: Days = Days::new(1);
const DAYS_6: Days = Days::new(6);

pub fn print_list(task_list: TaskList, data_dir_todo_output: PathBuf) {
    let output_file_name: &str = "datumos.md";
    let output_file_path: PathBuf = data_dir_todo_output.join(output_file_name);
    println!("Writing to output file '{}", output_file_path.clone().display());
    let mut file = match File::create(output_file_path.clone()) {
        Err(why) => {
            panic!("Couldn't open output file  '{}'\n{}", output_file_path.clone().display(), why);
        }
        Ok(file) => { file }
    };
    let file_ref: &mut File = &mut file;

    print_title(file_ref);

    let dt_now: DateTime<Local> = Local::now();
    let task_date_today = NaiveDate::from_ymd_opt(
        dt_now.year(),
        dt_now.month(),
        dt_now.day(),
    ).expect("Failed to create NaiveDate");
    let mut task_date_current = NaiveDate::from_ymd_opt(0, 1, 1)
        .expect("Failed to create NaiveDate");
    let mut tasks_iter = task_list.dated.iter();
    loop {
        let (task_date_ref, tasks) = match tasks_iter.next() {
            None => { break; }
            Some((task_dt_ref, tasks)) => {(task_dt_ref, tasks)}
        };
        if task_date_ref > &task_date_today { break; }

        if *task_date_ref != task_date_current {
            task_date_current = *task_date_ref;
            print_empty_line(file_ref);
            print_day_heading(task_date_ref, file_ref);
        }

        for task in tasks {
            print_dual(&format!("- [ ] {}", task), file_ref);
        }
    }

    print_year_heading(dt_now.year(), file_ref);
    let dt_last: NaiveDate = task_date_today.checked_add_months(MONTHS_12).expect("Failed adding months");
    let mut dt_next: NaiveDate = task_date_today;
    loop {
        dt_next = dt_next.checked_add_days(DAYS_1).expect("Failed adding day");

        if dt_next.weekday() == Weekday::Mon {
            let dt_sunday: NaiveDate = dt_next.checked_add_days(DAYS_6).expect("Failed adding days");
            if format!("{:?}", dt_sunday.iso_week()).ends_with("01") {
                print_year_heading(dt_sunday.year(), file_ref);
            }

            print_week_heading(&dt_next, &dt_sunday, file_ref)
        }

        let date_for_tasks = NaiveDate::from_ymd_opt(
            dt_next.year(),
            dt_next.month(),
            dt_next.day(),
        ).expect("Failed to create NaiveDate");
        match task_list.dated.get_key_value(&date_for_tasks) {
            None => {}
            Some((_, tasks)) => {
                print_day_heading(&date_for_tasks, file_ref);
                for task in tasks {
                    print_dual(&format!("- [ ] {}", task), file_ref);
                }
                print_empty_line(file_ref);
            }
        }

        if dt_next.eq(&dt_last) {
            break;
        }
    }

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

fn print_year_heading(year: i32, file_ref: &mut File) {
    print_empty_line(file_ref);
    {
        let line = String::from("---");
        print_dual(&line, file_ref);
    }
    {
        let line: String = format!("## {year}");
        print_dual(&line, file_ref);
    }
}

fn print_week_heading(dt_next: &NaiveDate, dt_sunday: &NaiveDate, file_ref: &mut File) {
    let mut date_range_display: String = format!("{} {}", words::month_abbrev(dt_next.month()), dt_next.day());

    if dt_next.month() == dt_sunday.month() {
        date_range_display = format!("{}-{}.",date_range_display, dt_sunday.day());
    } else {
        date_range_display = format!("{}. - {} {}.", date_range_display, words::month_abbrev(dt_sunday.month()), dt_sunday.day());
    }

    print_empty_line(file_ref);
    {
        let line: String = format!("#### {:?} ({})", dt_next.iso_week(), date_range_display);
        print_dual(&line, file_ref);
    }
    print_empty_line(file_ref);
}

fn print_day_heading(date_ref: &NaiveDate, file_ref: &mut File) {
    let weekday: String = words::day_abbrev(date_ref.weekday());
    let line: String = format!("{}-{:0>2}-{:0>2} ({})", date_ref.year(), date_ref.month(), date_ref.day(), weekday);
    print_dual(&line, file_ref);
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
    file_ref.flush().expect("Unable to flush write to output file.");
}
