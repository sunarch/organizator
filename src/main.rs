// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod config;
mod dated;
mod display_console;
mod display_file;
mod display_tui;
mod logging;
mod tasks;
mod time;
mod words;

// internal
use crate::tasks::task_data::TaskData;

const OPTION_DEBUG: &str = "--debug";
const OPTION_VERSION_SHORT: &str = "-v";
const OPTION_VERSION_LONG: &str = "--version";
const OPTION_DATED: &str = "--dated";
const OPTION_TODAY: &str = "--today";
const OPTION_TUI: &str = "--tui";

enum Intent {
    FileUpdateOnly,
    PrintDated,
    PrintToday,
    RunTUI,
}

fn main() {
    if std::env::args().nth(2).is_some() {
        println!("Too many arguments!");
        return;
    }

    let mut intent: Intent = Intent::FileUpdateOnly;

    if let Some(argument) = std::env::args().nth(1) {
        if argument == OPTION_DEBUG {
            logging::set_debug();
        } else if argument == OPTION_VERSION_SHORT || argument == OPTION_VERSION_LONG {
            print_version();
            return;
        } else if argument == OPTION_DATED {
            logging::set_warning();
            intent = Intent::PrintDated;
        } else if argument == OPTION_TODAY {
            logging::set_warning();
            intent = Intent::PrintToday;
        } else if argument == OPTION_TUI {
            intent = Intent::RunTUI;
        } else {
            println!("Unrecognized option: {argument}");
            return;
        }
    }

    let (data_dir_todo, data_dir_todo_output, _) = config::load_data_dirs();

    let task_data: TaskData = TaskData::load(data_dir_todo.as_ref());
    display_file::dated::print(&task_data, data_dir_todo_output.as_ref());

    match intent {
        Intent::FileUpdateOnly => {}
        Intent::PrintDated => display_console::dated::print(&task_data),
        Intent::PrintToday => display_console::dated::print_today(&task_data),
        Intent::RunTUI => display_tui::run().expect("Error running TUI"),
    }
}

fn print_version() {
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");
    println!("{name} {version}");
}
