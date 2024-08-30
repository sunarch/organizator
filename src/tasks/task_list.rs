// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fs::{self, DirEntry};
use std::path::{Display, PathBuf};
// dependencies
use chrono::NaiveDate;
// internal
use crate::tasks::task::Task;
use crate::tasks::{type_progressive, type_recurring, types};

pub struct TaskList {
    pub dated: BTreeMap<NaiveDate, Vec<Task>>,
    pub inactive: Vec<Task>,
}

impl TaskList {
    pub fn load(data_dir_todo: PathBuf) -> Self {
        let mut task_list = TaskList {
            dated: BTreeMap::new(),
            inactive: Default::default(),
        };

        let dir_path_progressive: PathBuf = data_dir_todo.join(type_progressive::DIR_NAME);
        Self::load_subdir(
            &dir_path_progressive,
            &mut task_list.dated,
            &mut task_list.inactive,
            &type_progressive::parse,
        );

        let dir_path_recurring: PathBuf = data_dir_todo.join(type_recurring::DIR_NAME);
        Self::load_subdir(
            &dir_path_recurring,
            &mut task_list.dated,
            &mut task_list.inactive,
            &type_recurring::parse,
        );

        return task_list;
    }

    fn load_subdir(
        todo_subdir: &PathBuf,
        tasks: &mut BTreeMap<NaiveDate, Vec<Task>>,
        tasks_inactive: &mut Vec<Task>,
        fn_parse: &types::FnParse,
    ) {
        let dir_path_display: Display = todo_subdir.display();
        if !todo_subdir.exists() {
            println!("Todo subdir '{dir_path_display}' not found, skipping.");
            return;
        }
        assert!(
            todo_subdir.is_dir(),
            "Todo subdir '{}' is not a directory!",
            dir_path_display
        );
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
                (task_date, task) = match fn_parse(&entry_path) {
                    None => {
                        continue;
                    }
                    Some((task_date, task)) => (task_date, task),
                };
                if task.active {
                    let date_task_list: &mut Vec<Task> = tasks.entry(task_date).or_default();
                    date_task_list.push(task);
                } else {
                    tasks_inactive.push(task);
                }
            }
        }
    }
}
