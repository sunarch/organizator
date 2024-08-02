// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::path::Path;
use directories_next::ProjectDirs;

const DIRS_QUALIFIER: &str = "dev";
const DIRS_ORGANIZATION: &str = "sunarch";


fn main() {
    let option_version_short: String = String::from("-v");
    let option_version_long: String = String::from("--version");

    match std::env::args().nth(1) {
        Some(argument) => {
            if argument == option_version_short || argument == option_version_long {
                let name = env!("CARGO_PKG_NAME");
                let version = env!("CARGO_PKG_VERSION");
                println!("{name} {version}");
            }
            else {
                println!("Unrecognized option: {argument}");
            }

        },
        None => {},
    }

    let project_dirs: ProjectDirs = match ProjectDirs::from(DIRS_QUALIFIER, DIRS_ORGANIZATION,  env!("CARGO_PKG_NAME")) {
        Some(value) => { value }
        None => {
            panic!("Unable to load project directory paths!")
        }
    };

    let config_dir: &Path = project_dirs.config_dir();
    match fs::create_dir_all(config_dir) {
        Ok(_) => {}
        Err(_) => {
            panic!("Unable to load create project config directory!")
        }
    }
}
