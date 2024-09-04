// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
// dependencies
use chrono::NaiveDate;
use serde;
// internal
use crate::tasks::task::contents::TaskContents;
use crate::tasks::task::meta::TaskMeta;
use crate::tasks::task::Task;
use crate::tasks::task_data::TaskAddable;
use crate::tasks::types;
use crate::time;

pub(crate) const DIR_NAME: &str = "marked-day";

#[derive(serde::Serialize, serde::Deserialize)]
struct MarkedDayItem {
    title: String,
    year_last_observed: i32,

    //#[serde(default = "types::default_zero_i32")]
    year: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MarkedDayDay {
    month: u32,
    day: u32,
    items: Vec<MarkedDayItem>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    mark_title: String,
    description: String,
    days: Vec<MarkedDayDay>,
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

            let is_done: bool = date_last_observed > date_current_year;

            let subtask_current_year: TaskContents = TaskContents {
                title: subtask_title(&item.title, item.year, task_data.year_current()),
                note: Default::default(),
                active: !is_done,
            };

            subtasks_current_year.push(subtask_current_year);

            if is_done {
                let subtask_next_year: TaskContents = TaskContents {
                    title: subtask_title(&item.title, item.year, task_data.year_next()),
                    note: Default::default(),
                    active: true,
                };

                subtasks_next_year.push(subtask_next_year);
            }
        }

        if !subtasks_current_year
            .iter()
            .map(|subtask| !subtask.active)
            .reduce(|inactive_self, inactive_other| inactive_self && inactive_other)
            .expect("Failed to reduce subtasks to all inactive bool")
        {
            let task_current_year: Task = Task {
                meta: TaskMeta {
                    frequency: Default::default(),
                    subtasks: subtasks_current_year,
                },
                contents: TaskContents {
                    title: data.mark_title.clone(),
                    note: Default::default(),
                    active: true,
                },
            };

            task_data.add_task(date_current_year, task_current_year);
        }

        if !subtasks_next_year.is_empty() {
            let task_next_year: Task = Task {
                meta: TaskMeta {
                    frequency: Default::default(),
                    subtasks: subtasks_next_year,
                },
                contents: TaskContents {
                    title: data.mark_title.clone(),
                    note: Default::default(),
                    active: true,
                },
            };

            task_data.add_task(date_next_year, task_next_year);
        }
    }
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
