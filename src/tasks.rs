// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fmt;
use std::fs::{self, DirEntry};
use std::path::{PathBuf, Display};

use chrono::{Datelike, DateTime, Local, NaiveDate};

const DIR_NAME_PROGRESSIVE: &str = "progressive";
const DIR_NAME_RECURRING: &str = "recurring";
const DIR_NAME_SERIAL: &str = "serial";

pub struct Task {
    frequency: String,
    title: String,
    note: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        if self.frequency.len() > 0 {
            display = format!("{} - ", self.frequency);
        }
        display.push_str(self.title.as_str());
        if self.note.len() > 0 {
            display = format!("{} ({})", display, self.note);
        }
        return write!(f, "{}", display);
    }
}

impl Task {
    fn parse_progressive(file_path: &PathBuf) -> (NaiveDate, Self) {
        let timestamp: DateTime<Local> = Local::now();
        return (
            NaiveDate::from_ymd_opt(
                timestamp.year(),
                timestamp.month(),
                timestamp.day()
            ).expect("Failed to create NaiveDate"),
            Task {
                frequency: String::from("frequency"),
                title: String::from(
                    file_path.file_name()
                        .expect("Failed to parse filename.")
                        .to_str()
                        .expect("Failed to convert string.")
                ),
                note: String::from("note"),
            },
        )
    }

    fn parse_recurring(file_path: &PathBuf) -> (NaiveDate, Self) {
        let timestamp: DateTime<Local> = Local::now();
        return (
            NaiveDate::from_ymd_opt(
                timestamp.year(),
                timestamp.month(),
                timestamp.day()
            ).expect("Failed to create NaiveDate"),
            Task {
                frequency: String::from("frequency"),
                title: String::from(
                    file_path.file_name()
                        .expect("Failed to parse filename.")
                        .to_str()
                        .expect("Failed to convert string.")
                ),
                note: String::from("note"),
            },
        )
    }

    fn parse_serial(file_path: &PathBuf) -> (NaiveDate, Self) {
        let timestamp: DateTime<Local> = Local::now();
        return (
            NaiveDate::from_ymd_opt(
                timestamp.year(),
                timestamp.month(),
                timestamp.day()
            ).expect("Failed to create NaiveDate"),
            Task {
                frequency: String::from("frequency"),
                title: String::from(
                    file_path.file_name()
                        .expect("Failed to parse filename.")
                        .to_str()
                        .expect("Failed to convert string.")
                ),
                note: String::from("note"),
            },
        )
    }
}

pub struct TaskList {
    pub dated: BTreeMap<NaiveDate, Vec<Task>>,
}

impl TaskList {
    pub fn load(data_dir_todo: PathBuf) -> Self {
        let mut task_list = TaskList {
            dated: BTreeMap::new()
        };

        let dir_path_progressive: PathBuf = data_dir_todo.join(DIR_NAME_PROGRESSIVE);
        Self::load_subdir(&dir_path_progressive, &mut task_list.dated, &Task::parse_progressive);

        let dir_path_recurring: PathBuf = data_dir_todo.join(DIR_NAME_RECURRING);
        Self::load_subdir(&dir_path_recurring, &mut task_list.dated, &Task::parse_recurring);

        let dir_path_serial: PathBuf = data_dir_todo.join(DIR_NAME_SERIAL);
        Self::load_subdir(&dir_path_serial, &mut task_list.dated, &Task::parse_serial);

        return task_list;
    }

    fn load_subdir<FnParse>(
        todo_subdir: &PathBuf,
        tasks: &mut BTreeMap<NaiveDate, Vec<Task>>,
        fn_parse: FnParse,
    ) where
        FnParse: Fn(&PathBuf) -> (NaiveDate, Task)
    {
        let dir_path_display: Display = todo_subdir.display();
        if !todo_subdir.exists() {
            println!("Todo subdir '{dir_path_display}' not found, skipping.");
            return;
        }
        assert!(todo_subdir.is_dir(), "Todo subdir '{}' is not a directory!", dir_path_display);
        println!("Found todo subdir '{dir_path_display}'");

        let mut task_date: NaiveDate;
        let mut task: Task;

        for entry in fs::read_dir(todo_subdir).expect("Failed to iterate todo subdir.") {
            let entry: DirEntry = entry.expect("Failed to iterate dir entry.");
            let entry_path: PathBuf = entry.path();
            let entry_path_display: Display = entry_path.display();
            if entry_path.is_dir() {
                println!("Warning: dir inside todo subdir: '{entry_path_display}'");
            } else {
                (task_date, task) = fn_parse(&entry_path);
                if !tasks.contains_key(&task_date) {
                    tasks.insert(task_date, vec!());
                }
                tasks.get_mut(&task_date)
                    .expect("Failed to load task list day.")
                    .push(task);
            }
        }
    }
}
