// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::Weekday;

pub(crate) const DATED_TITLE: &str = "ismétlődő - dátumos";

pub(crate) fn day_abbrev(weekday: Weekday) -> String {
    return match weekday {
        Weekday::Mon => {String::from("H")}
        Weekday::Tue => {String::from("K")}
        Weekday::Wed => {String::from("Sze")}
        Weekday::Thu => {String::from("Cs")}
        Weekday::Fri => {String::from("P")}
        Weekday::Sat => {String::from("Szo")}
        Weekday::Sun => {String::from("V")}
    };
}
