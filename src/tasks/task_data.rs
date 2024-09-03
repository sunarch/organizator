// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fs::{self, DirEntry};
use std::path::{Display, Path, PathBuf};
// dependencies
use chrono::{Datelike, NaiveDate, NaiveWeek};
// internal
use crate::logging;
use crate::tasks::task::Task;
use crate::tasks::{type_progressive, type_recurring, type_simple, types};
use crate::time;

pub struct TaskData {
    pub dates: TaskDates,
    pub sections: TaskSections,
}

impl TaskData {
    pub fn load(data_dir_todo: &Path) -> Self {
        let dates: TaskDates = TaskDates::create();
        let sections: TaskSections = TaskSections::load(data_dir_todo, &dates);
        return TaskData { dates, sections };
    }
}

pub struct TaskDates {
    pub today: NaiveDate,
    pub current_year: i32,
    pub next_year: i32,
    pub first_in_dated_full_weeks: NaiveDate,
    pub last_dated: NaiveDate,
    pub dated_weeks_current_year: Vec<NaiveWeek>,
    pub dated_weeks_next_year: Vec<NaiveWeek>,
}

enum DatedWeeksPart {
    CurrentYear,
    NextYear,
}

impl TaskDates {
    pub fn create() -> Self {
        let today: NaiveDate = time::today();
        let first_in_dated_full_weeks: NaiveDate = time::next_monday(&today);
        let last_dated: NaiveDate = time::first_sunday_after_12_months(&today);

        let mut year_of_week: DatedWeeksPart = DatedWeeksPart::CurrentYear;
        let mut dated_weeks_current_year: Vec<NaiveWeek> = Default::default();
        let mut dated_weeks_next_year: Vec<NaiveWeek> = Default::default();
        let mut dated_weeks_iter_date: NaiveDate = first_in_dated_full_weeks;
        while dated_weeks_iter_date < last_dated {
            match year_of_week {
                DatedWeeksPart::CurrentYear => {
                    dated_weeks_current_year.push(time::week_of_day(&dated_weeks_iter_date));
                }
                DatedWeeksPart::NextYear => {
                    dated_weeks_next_year.push(time::week_of_day(&dated_weeks_iter_date));
                }
            }
            dated_weeks_iter_date = time::increment_by_one_week(&dated_weeks_iter_date);
            if time::is_day_in_first_week_of_year(&dated_weeks_iter_date) {
                year_of_week = DatedWeeksPart::NextYear;
            }
        }

        return TaskDates {
            today,
            current_year: today.year(),
            next_year: today.year() + 1,
            first_in_dated_full_weeks,
            last_dated,
            dated_weeks_current_year,
            dated_weeks_next_year,
        };
    }
}

pub struct TaskSections {
    pub overdue: BTreeMap<NaiveDate, Vec<Task>>,
    pub today: Vec<Task>,
    pub dated_current_week: BTreeMap<NaiveDate, Vec<Task>>,
    pub dated: BTreeMap<NaiveDate, Vec<Task>>,
    pub later: BTreeMap<NaiveDate, Vec<Task>>,
    pub inactive: Vec<Task>,
}

impl TaskSections {
    fn load(data_dir_todo: &Path, task_dates: &TaskDates) -> Self {
        let mut task_sections = TaskSections {
            overdue: Default::default(),
            today: Default::default(),
            dated_current_week: Default::default(),
            dated: Default::default(),
            later: Default::default(),
            inactive: Default::default(),
        };

        let dir_path_progressive: PathBuf = data_dir_todo.join(type_progressive::DIR_NAME);
        Self::load_subdir(
            &dir_path_progressive,
            &mut task_sections,
            &type_progressive::parse,
            task_dates,
        );

        let dir_path_recurring: PathBuf = data_dir_todo.join(type_recurring::DIR_NAME);
        Self::load_subdir(
            &dir_path_recurring,
            &mut task_sections,
            &type_recurring::parse,
            task_dates,
        );

        let dir_path_simple: PathBuf = data_dir_todo.join(type_simple::DIR_NAME);
        Self::load_subdir(
            &dir_path_simple,
            &mut task_sections,
            &type_simple::parse,
            task_dates,
        );

        return task_sections;
    }

    fn load_subdir(
        todo_subdir: &Path,
        task_sections: &mut TaskSections,
        fn_parse: &types::FnParse,
        task_dates: &TaskDates,
    ) {
        let dir_path_display: Display = todo_subdir.display();
        if !todo_subdir.exists() {
            logging::warning(format!(
                "Todo subdir '{dir_path_display}' not found, skipping."
            ));
            return;
        }
        if !todo_subdir.is_dir() {
            logging::warning(format!(
                "Todo subdir '{dir_path_display}' is not a directory, skipping."
            ));
            return;
        }
        logging::info(format!("Found todo subdir '{dir_path_display}'"));

        let mut task_date: NaiveDate;
        let mut task: Task;

        for entry in fs::read_dir(todo_subdir).expect("Failed to iterate todo subdir.") {
            let entry: DirEntry = entry.expect("Failed to iterate dir entry.");
            let entry_path: PathBuf = entry.path();
            let entry_path_display: Display = entry_path.display();
            if entry_path.is_dir() {
                logging::warning(format!("Dir inside todo subdir: '{entry_path_display}'"));
            } else {
                (task_date, task) = match fn_parse(&entry_path) {
                    None => {
                        continue;
                    }
                    Some((task_date, task)) => (task_date, task),
                };

                if !task.active {
                    task_sections.inactive.push(task);
                    continue;
                }

                if task_date < task_dates.today {
                    let tasks_overdue: &mut Vec<Task> =
                        task_sections.overdue.entry(task_date).or_default();
                    tasks_overdue.push(task);
                } else if task_date == task_dates.today {
                    task_sections.today.push(task);
                } else if task_date > task_dates.today
                    && task_date < task_dates.first_in_dated_full_weeks
                {
                    let tasks_dated_current_week: &mut Vec<Task> = task_sections
                        .dated_current_week
                        .entry(task_date)
                        .or_default();
                    tasks_dated_current_week.push(task);
                } else if task_date > task_dates.last_dated {
                    let tasks_later: &mut Vec<Task> =
                        task_sections.later.entry(task_date).or_default();
                    tasks_later.push(task);
                } else {
                    let tasks_dated: &mut Vec<Task> =
                        task_sections.dated.entry(task_date).or_default();
                    tasks_dated.push(task);
                }
            }
        }

        task_sections.sort_task_lists()
    }

    fn sort_task_lists(&mut self) {
        for task_list in self.overdue.values_mut() {
            task_list.sort();
        }
        {
            self.today.sort();
        }
        for task_list in self.dated_current_week.values_mut() {
            task_list.sort();
        }
        for task_list in self.dated.values_mut() {
            task_list.sort();
        }
        for task_list in self.later.values_mut() {
            task_list.sort();
        }
        {
            self.inactive.sort();
        }
    }
}
