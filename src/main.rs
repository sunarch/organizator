// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod config;
mod dated;
mod logging;
mod tasks;
mod time;
mod tui;
mod words;

// internal
use crate::tasks::task_data::TaskData;

const OPTION_DEBUG: &str = "--debug";
const OPTION_VERSION_SHORT: &str = "-v";
const OPTION_VERSION_LONG: &str = "--version";
const OPTION_DATED: &str = "--dated";
const OPTION_TUI: &str = "--tui";

fn main() {
    if std::env::args().nth(2).is_some() {
        println!("Too many arguments!");
        return;
    }

    let mut print_dated: bool = false;
    let mut run_tui: bool = false;

    if let Some(argument) = std::env::args().nth(1) {
        if argument == OPTION_DEBUG {
            logging::set_debug();
        } else if argument == OPTION_VERSION_SHORT || argument == OPTION_VERSION_LONG {
            print_version();
            return;
        } else if argument == OPTION_DATED {
            logging::set_warning();
            print_dated = true;
        } else if argument == OPTION_TUI {
            run_tui = true;
        } else {
            panic!("Unrecognized option: {argument}");
        }
    }

    let (data_dir_todo, data_dir_todo_output, _) = config::load_data_dirs();

    let task_data: TaskData = TaskData::load(data_dir_todo.as_ref());
    dated::print_to_file(&task_data, data_dir_todo_output.as_ref());

    if print_dated {
        dated::print_to_console(&task_data);
        return;
    }

    if run_tui {
        tui::run().expect("Error running TUI");
        return;
    }
}

fn print_version() {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");
    println!("{name} {version}");
}
