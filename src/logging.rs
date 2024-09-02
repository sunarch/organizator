// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

use std::sync::atomic::{AtomicU8, Ordering};

const LOG_LEVEL_NONE: u8 = 0;
const LOG_LEVEL_ERROR: u8 = 1;
const LOG_LEVEL_WARNING: u8 = 2;
const LOG_LEVEL_INFO: u8 = 3;
const LOG_LEVEL_DEBUG: u8 = 4;

static LOG_LEVEL: AtomicU8 = AtomicU8::new(LOG_LEVEL_INFO);

pub fn error(msg: String) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= LOG_LEVEL_ERROR {
        println!("[ERROR  ] {}", msg);
    }
}

pub fn warning(msg: String) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= LOG_LEVEL_WARNING {
        println!("[WARNING] {}", msg);
    }
}

pub fn info(msg: String) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= LOG_LEVEL_INFO {
        println!("[INFO   ] {}", msg);
    }
}

pub fn debug(msg: String) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= LOG_LEVEL_DEBUG {
        println!("[DEBUG  ] {}", msg);
    }
}

pub fn set_debug() {
    LOG_LEVEL.store(LOG_LEVEL_DEBUG, Ordering::Relaxed);
}
