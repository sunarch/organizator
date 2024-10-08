// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
// dependencies
use chrono::NaiveDate;
// internal
use crate::tasks::task::Task;

pub(crate) struct TaskSections {
    pub(crate) overdue: BTreeMap<NaiveDate, Vec<Task>>,
    pub(crate) today: Vec<Task>,
    pub(crate) rest_of_the_week: BTreeMap<NaiveDate, Vec<Task>>,
    pub(crate) dated: BTreeMap<NaiveDate, Vec<Task>>,
    pub(crate) later: BTreeMap<NaiveDate, Vec<Task>>,
    pub(crate) inactive: Vec<Task>,
}

impl Default for TaskSections {
    fn default() -> Self {
        return TaskSections {
            overdue: Default::default(),
            today: Default::default(),
            rest_of_the_week: Default::default(),
            dated: Default::default(),
            later: Default::default(),
            inactive: Default::default(),
        };
    }
}

impl TaskSections {
    pub(crate) fn sort_task_lists(&mut self) {
        for task_list in self.overdue.values_mut() {
            task_list.sort();
        }
        {
            self.today.sort();
        }
        for task_list in self.rest_of_the_week.values_mut() {
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
