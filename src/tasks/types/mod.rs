// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod type_marked_day;
pub(crate) mod type_progressive;
pub(crate) mod type_recurring;
pub(crate) mod type_simple;

use std::fs::File;
use std::path::Path;
// dependencies
use serde::Deserialize;
use serde_json::from_reader;
// internal
use crate::logging;
use crate::tasks::data::TaskAddable;

pub(crate) type FnLoadTaskType = dyn Fn(&Path, &mut dyn TaskAddable);

pub(crate) fn default_true() -> bool {
    return true;
}

pub(crate) fn default_false() -> bool {
    return false;
}

pub(crate) fn default_zero_i32() -> i32 {
    return 0;
}

pub(crate) fn default_vec<T>() -> Vec<T> {
    return Default::default();
}

pub(crate) fn load<Data: for<'de> Deserialize<'de>>(file_path: &Path) -> Option<Data> {
    let file = match File::open(file_path) {
        Err(why) => {
            logging::error(format!(
                "Couldn't open todo file '{}' \n{}",
                file_path.display(),
                why
            ));
            return None;
        }
        Ok(file) => file,
    };

    match from_reader(file) {
        Err(why) => {
            logging::error(format!(
                "Couldn't parse todo file '{}' \n{}",
                file_path.display(),
                why
            ));
            return None;
        }
        Ok(data) => Some(data),
    }
}
