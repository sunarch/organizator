// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// internal
use crate::dated;
use crate::tasks::task_data::TaskData;

pub(crate) fn print(task_data: &TaskData) {
    dated::print_list(task_data, &mut None);
}

pub(crate) fn print_today(task_data: &TaskData) {
    dated::print_part_today(&task_data.dates.today, &task_data.sections.today, &mut None);
}
