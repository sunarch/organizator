// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::Weekday;

pub(crate) const DATED_TITLE: &str = "ismétlődő - dátumos";

pub(crate) fn month_name(number: u32) -> String {
    return match number {
        1 => { String::from("jan.") }
        2 => { String::from("feb.") }
        3 => { String::from("márc.") }
        4 => { String::from("ápr.") }
        5 => { String::from("máj.") }
        6 => { String::from("jún.") }
        7 => { String::from("júl.") }
        8 => { String::from("aug.") }
        9 => { String::from("szept.") }
        10 => { String::from("okt.") }
        11 => { String::from("nov.") }
        12 => { String::from("dec.") }
        _ => {
            panic!("Illegal month number: '{}'", number);
        }
    };
}

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
