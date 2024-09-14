// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

mod log_level;

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
// internal
use crate::logging::log_level::LogLevel;
use crate::time::timestamp;

const MSG_LOCK_FAIL_READ: &str = "Failed to acquire lock to read log level.";
const MSG_LOCK_FAIL_WRITE: &str = "Failed to acquire lock to write log level.";

static LOG_LEVEL: RwLock<LogLevel> = RwLock::new(LogLevel::Info);

pub fn set_warning() {
    let mut lock: RwLockWriteGuard<LogLevel> = LOG_LEVEL.write().expect(MSG_LOCK_FAIL_WRITE);
    *lock = LogLevel::Warning;
}

pub fn set_debug() {
    let mut lock: RwLockWriteGuard<LogLevel> = LOG_LEVEL.write().expect(MSG_LOCK_FAIL_WRITE);
    *lock = LogLevel::Debug;
}

pub fn error(msg: String) {
    let lock: RwLockReadGuard<LogLevel> = LOG_LEVEL.read().expect(MSG_LOCK_FAIL_READ);
    if *lock >= LogLevel::Error {
        log("ERROR  ", &msg);
    }
}

pub fn warning(msg: String) {
    let lock: RwLockReadGuard<LogLevel> = LOG_LEVEL.read().expect(MSG_LOCK_FAIL_READ);
    if *lock >= LogLevel::Warning {
        log("WARNING", &msg);
    }
}

pub fn info(msg: String) {
    let lock: RwLockReadGuard<LogLevel> = LOG_LEVEL.read().expect(MSG_LOCK_FAIL_READ);
    if *lock >= LogLevel::Info {
        log("INFO   ", &msg);
    }
}

pub fn debug(msg: String) {
    let lock: RwLockReadGuard<LogLevel> = LOG_LEVEL.read().expect(MSG_LOCK_FAIL_READ);
    if *lock >= LogLevel::Debug {
        log("DEBUG  ", &msg);
    }
}

fn log(prefix: &str, message: &String) {
    println!("[{}][{}] {}", timestamp::current_clock(), prefix, message);
}
