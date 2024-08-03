// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod config;


fn main() {
    let option_version_short: String = String::from("-v");
    let option_version_long: String = String::from("--version");

    if let Some(argument) = std::env::args().nth(1) {
        if argument == option_version_short || argument == option_version_long {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            println!("{name} {version}");
            return;
        } else {
            panic!("Unrecognized option: {argument}");
        }
    }

    (_, _, _) = config::load_data_dirs();
}
