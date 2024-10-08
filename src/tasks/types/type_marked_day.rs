// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
// internal
use crate::tasks::data::TaskAddable;
use crate::tasks::task::contents::{TaskContents, TaskVisibility};
use crate::tasks::task::meta::{TaskMeta, TaskMetaDisplayOptions};
use crate::tasks::task::Task;
use crate::tasks::types;
use crate::time;

pub(crate) const DIR_NAME: &str = "marked-day";

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    mark_title: String,
    description: Option<String>,
    days: Vec<DataDay>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DataDay {
    month: u32,
    day: u32,
    items: Vec<DataItem>,
}

#[derive(Serialize, Deserialize)]
struct DataItem {
    title: String,
    note: Option<String>,
    year: Option<i32>,
    year_last_observed: i32,
    hidden: Option<bool>,
}

pub(crate) fn load(file_path: &Path, task_data: &mut dyn TaskAddable) {
    let data: Data = match types::load(file_path) {
        None => {
            return;
        }
        Some(data) => data,
    };

    for day in data.days {
        let date_current_year: NaiveDate = match time::parsing::date_opt_from_ymd(
            task_data.year_current(),
            day.month,
            day.day,
            "marked day task (current year)",
            data.mark_title.as_str(),
        ) {
            None => {
                continue;
            }
            Some(date) => date,
        };

        let date_next_year = match time::parsing::date_opt_from_ymd(
            task_data.year_next(),
            day.month,
            day.day,
            "marked day task (next year)",
            data.mark_title.as_str(),
        ) {
            None => {
                continue;
            }
            Some(date) => date,
        };

        let mut subtasks_current_year: Vec<TaskContents> = Default::default();
        let mut subtasks_next_year: Vec<TaskContents> = Default::default();

        for item in day.items {
            if item.hidden == Some(true) {
                continue;
            }

            let subtask_note: String = item.note.unwrap_or_else(Default::default);

            let date_last_observed = match time::parsing::date_opt_from_ymd(
                item.year_last_observed,
                day.month,
                day.day,
                "marked day task (last observed year)",
                format!("{}) ({}", item.title, data.mark_title).as_str(),
            ) {
                None => {
                    continue;
                }
                Some(date) => date,
            };

            let is_done_for_current_year: bool = date_last_observed >= date_current_year;

            let subtask_current_year: TaskContents = TaskContents {
                title: subtask_title(&item.title, item.year, task_data.year_current()),
                note: subtask_note.clone(),
                is_done: is_done_for_current_year,
                visibility: TaskVisibility::Visible,
            };

            subtasks_current_year.push(subtask_current_year);

            if is_done_for_current_year {
                let subtask_next_year: TaskContents = TaskContents {
                    title: subtask_title(&item.title, item.year, task_data.year_next()),
                    note: subtask_note.clone(),
                    is_done: false,
                    visibility: TaskVisibility::Visible,
                };

                subtasks_next_year.push(subtask_next_year);
            }
        }

        let today: NaiveDate = task_data.date_today();

        if !subtasks_current_year
            .iter()
            .map(|subtask| subtask.is_done)
            .all(|is_done| is_done)
            && !subtasks_current_year.is_empty()
        {
            let overdue: bool = date_current_year < today;
            let is_today: bool = date_current_year == today;
            let task_current_year: Task =
                create_task(subtasks_current_year, overdue, is_today, &data.mark_title);
            task_data.add_task(date_current_year, task_current_year);
        }

        if !subtasks_next_year.is_empty() {
            let overdue: bool = date_next_year < today;
            let is_today: bool = date_next_year == today;
            let task_next_year: Task =
                create_task(subtasks_next_year, overdue, is_today, &data.mark_title);
            task_data.add_task(date_next_year, task_next_year);
        }
    }
}

fn create_task(
    subtasks: Vec<TaskContents>,
    overdue: bool,
    is_today: bool,
    mark_title: &str,
) -> Task {
    return Task {
        meta: TaskMeta {
            frequency: Default::default(),
            time_of_day: Default::default(),
            overdue,
            subtasks,
            display_options: TaskMetaDisplayOptions {
                overdue_mark: is_today,
            },
        },
        contents: TaskContents {
            title: mark_title.to_string(),
            note: Default::default(),
            is_done: false,
            visibility: TaskVisibility::Visible,
        },
    };
}

fn subtask_title(item_title: &String, origin_year_opt: Option<i32>, task_year: i32) -> String {
    return match origin_year_opt {
        None => item_title.clone(),
        Some(origin_year) => {
            let year_diff: i32 = task_year - origin_year;
            let years_plural: &str = if year_diff > 1 { "s" } else { "" };

            format!(
                "{} ({} year{} since {})",
                item_title, year_diff, years_plural, origin_year
            )
        }
    };
}
