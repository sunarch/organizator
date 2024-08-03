// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::path::{Display, Path, PathBuf};

use directories_next::ProjectDirs;

const DIRS_QUALIFIER: &str = "dev";
const DIRS_ORGANIZATION: &str = "sunarch";


pub fn load_dir() -> PathBuf {

    let project_dirs: ProjectDirs = match ProjectDirs::from(DIRS_QUALIFIER, DIRS_ORGANIZATION, env!("CARGO_PKG_NAME")) {
        Some(value) => { value }
        None => {
            panic!("Unable to load project directory paths!")
        }
    };

    let config_dir: &Path = project_dirs.config_dir();
    let config_dir_display: Display = config_dir.display();
    if config_dir.exists() {
        println!("Found config dir '{config_dir_display}'")
    }
    else {
        match fs::create_dir_all(config_dir) {
            Ok(_) => {
                println!("Created config dir '{config_dir_display}'")
            }
            Err(_) => {
                panic!("Unable to load or create project config directory!")
            }
        }
    }

    return config_dir.to_path_buf();
}
