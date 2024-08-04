// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod config;
mod dated;
mod tui;


fn main() {
    let option_version_short: String = String::from("-v");
    let option_version_long: String = String::from("--version");

    let option_dated: String = String::from("--dated");
    let mut show_dated: bool = false;

    let option_tui: String = String::from("--tui");
    let mut run_tui: bool = false;

    if let Some(argument) = std::env::args().nth(1) {
        if argument == option_version_short || argument == option_version_long {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            println!("{name} {version}");
            return;
        }
        else if argument == option_dated {
            show_dated = true;
        }
        else if argument == option_tui {
            run_tui = true;
        }
        else {
            panic!("Unrecognized option: {argument}");
        }
    }

    (_, _, _) = config::load_data_dirs();

    if run_tui {
        tui::run().expect("Error running TUI");
    }

    if show_dated {
        dated::print_list();
    }
}
