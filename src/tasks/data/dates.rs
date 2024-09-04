// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use chrono::{Datelike, NaiveDate, NaiveWeek};
// internal
use crate::time;

pub(crate) struct TaskDates {
    pub(crate) today: NaiveDate,
    pub(crate) current_year: i32,
    pub(crate) next_year: i32,
    pub(crate) first_in_dated_full_weeks: NaiveDate,
    pub(crate) last_dated: NaiveDate,
    pub(crate) dated_weeks_current_year: Vec<NaiveWeek>,
    pub(crate) dated_weeks_next_year: Vec<NaiveWeek>,
}

enum DatedWeeksPart {
    CurrentYear,
    NextYear,
}

impl TaskDates {
    pub(crate) fn create() -> Self {
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
