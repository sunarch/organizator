// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod config;
mod dated;
mod task_types;
mod tasks;
mod time;
mod tui;
mod words;

fn main() {
    let option_version_short = String::from("-v");
    let option_version_long = String::from("--version");

    let option_dated = String::from("--dated");
    let mut show_dated: bool = false;

    let option_tui = String::from("--tui");
    let mut run_tui: bool = false;

    if let Some(argument) = std::env::args().nth(1) {
        if argument == option_version_short || argument == option_version_long {
            let name: &str = env!("CARGO_PKG_NAME");
            let version: &str = env!("CARGO_PKG_VERSION");
            println!("{name} {version}");
            return;
        } else if argument == option_dated {
            show_dated = true;
        } else if argument == option_tui {
            run_tui = true;
        } else {
            panic!("Unrecognized option: {argument}");
        }
    }

    let (data_dir_todo, data_dir_todo_output, _) = config::load_data_dirs();

    if run_tui {
        tui::run().expect("Error running TUI");
    }

    let task_list: tasks::TaskList = tasks::TaskList::load(data_dir_todo);

    if show_dated {
        dated::print_list(task_list, data_dir_todo_output);
    }
}
