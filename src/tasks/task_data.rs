// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::{self, DirEntry};
use std::path::{Display, Path, PathBuf};
// dependencies
use chrono::NaiveDate;
// internal
use crate::logging;
use crate::tasks::task::Task;
use crate::tasks::task_dates::TaskDates;
use crate::tasks::task_sections::TaskSections;
use crate::tasks::types::FnLoadTaskType;
use crate::tasks::{type_progressive, type_recurring, type_simple};

pub struct TaskData {
    pub dates: TaskDates,
    pub sections: TaskSections,
}

impl TaskData {
    pub fn load(data_dir_todo: &Path) -> Self {
        let dates: TaskDates = TaskDates::create();
        let sections: TaskSections = Default::default();
        let mut data: TaskData = TaskData { dates, sections };
        {
            let dir_path: PathBuf = data_dir_todo.join(type_progressive::DIR_NAME);
            data.load_subdir(&dir_path, &type_progressive::load_one);
        }
        {
            let dir_path: PathBuf = data_dir_todo.join(type_recurring::DIR_NAME);
            data.load_subdir(&dir_path, &type_recurring::load_one);
        }
        {
            let dir_path: PathBuf = data_dir_todo.join(type_simple::DIR_NAME);
            data.load_subdir(&dir_path, &type_simple::load);
        }
        return data;
    }

    fn load_subdir(&mut self, todo_subdir: &Path, fn_load: &FnLoadTaskType) {
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

        for entry in fs::read_dir(todo_subdir).expect("Failed to iterate todo subdir.") {
            let entry: DirEntry = entry.expect("Failed to iterate dir entry.");
            let entry_path: PathBuf = entry.path();
            let entry_path_display: Display = entry_path.display();
            if entry_path.is_dir() {
                logging::warning(format!("Dir inside todo subdir: '{entry_path_display}'"));
                continue;
            }
            fn_load(&entry_path, self);
        }

        self.sections.sort_task_lists()
    }
}

pub(crate) trait TaskAddable {
    fn add_task(&mut self, task_date: NaiveDate, task: Task);
}

impl TaskAddable for TaskData {
    fn add_task(&mut self, task_date: NaiveDate, task: Task) {
        let task_sections: &mut TaskSections = &mut self.sections;
        let task_dates: &TaskDates = &self.dates;

        if !task.active {
            task_sections.inactive.push(task);
            return;
        }

        if task_date < task_dates.today {
            let tasks_overdue: &mut Vec<Task> = task_sections.overdue.entry(task_date).or_default();
            tasks_overdue.push(task);
        } else if task_date == task_dates.today {
            task_sections.today.push(task);
        } else if task_date > task_dates.today && task_date < task_dates.first_in_dated_full_weeks {
            let tasks_dated_current_week: &mut Vec<Task> = task_sections
                .dated_current_week
                .entry(task_date)
                .or_default();
            tasks_dated_current_week.push(task);
        } else if task_date > task_dates.last_dated {
            let tasks_later: &mut Vec<Task> = task_sections.later.entry(task_date).or_default();
            tasks_later.push(task);
        } else {
            let tasks_dated: &mut Vec<Task> = task_sections.dated.entry(task_date).or_default();
            tasks_dated.push(task);
        }
    }
}
