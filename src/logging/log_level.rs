// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
pub(super) enum LogLevel {
    None,
    Error,
    Warning,
    Info,
    Debug,
}

impl Default for LogLevel {
    fn default() -> Self {
        return LogLevel::Info;
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.value().cmp(&other.value());
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl LogLevel {
    fn value(&self) -> u8 {
        return match self {
            LogLevel::None => 0,
            LogLevel::Error => 1,
            LogLevel::Warning => 2,
            LogLevel::Info => 3,
            LogLevel::Debug => 4,
        };
    }
}
