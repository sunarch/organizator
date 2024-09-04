// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::NaiveDate;
// internal
use crate::logging;

pub fn date_opt_from_ymd(
    year: i32,
    month: u32,
    day: u32,
    note_place: &str,
    note_item: &str,
) -> Option<NaiveDate> {
    let date_opt: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, month, day);
    if date_opt.is_none() {
        logging::error(format!(
            "Failed to convert date from Y-M-D in {}: '{}'-'{}'-'{}' ({})",
            note_place, year, month, day, note_item
        ));
    };
    return date_opt;
}
