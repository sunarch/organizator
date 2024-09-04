// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::{Read, Stdin, Stdout, Write};
use std::path::{Display, Path, PathBuf};
use std::{fs, io};
// dependencies
use directories_next::ProjectDirs;
// internal
use crate::logging;

const DIRS_QUALIFIER: &str = "dev";
const DIRS_ORGANIZATION: &str = "sunarch";

fn load_dir() -> PathBuf {
    let project_dirs: ProjectDirs =
        ProjectDirs::from(DIRS_QUALIFIER, DIRS_ORGANIZATION, env!("CARGO_PKG_NAME"))
            .unwrap_or_else(|| panic!("Unable to load project directory paths!"));

    let config_dir: &Path = project_dirs.config_dir();
    let config_dir_display: Display = config_dir.display();
    if config_dir.exists() {
        logging::info(format!("Found config dir '{config_dir_display}'"));
    } else {
        match fs::create_dir_all(config_dir) {
            Ok(_) => {
                logging::info(format!("Created config dir '{config_dir_display}'"));
            }
            Err(_) => {
                panic!("Unable to load or create project config directory!");
            }
        }
    }

    return config_dir.to_path_buf();
}

fn load_data_dir_single(
    config_dir: &Path,
    config_file_name: &str,
    config_file_purpose: &str,
) -> PathBuf {
    let config_file_path: PathBuf = config_dir.join(config_file_name);
    let mut config_content_is_ok: bool = false;
    let mut data_dir: PathBuf = Default::default();

    if config_file_path
        .try_exists()
        .expect("Checking config file existence unsuccessful.")
    {
        let mut file = match File::open(config_file_path.clone()) {
            Err(why) => {
                panic!(
                    "Couldn't open config file for {} ({})\n{}",
                    config_file_purpose,
                    config_file_path.clone().display(),
                    why
                );
            }
            Ok(file) => file,
        };

        let mut input = String::new();
        match file.read_to_string(&mut input) {
            Err(why) => {
                panic!(
                    "Couldn't read config file for {} ({})\n{}",
                    config_file_purpose,
                    config_file_path.clone().display(),
                    why
                );
            }
            Ok(_) => {
                data_dir = PathBuf::from(input);

                if data_dir.exists() {
                    config_content_is_ok = true;
                }
            }
        }
    }

    if !config_content_is_ok {
        let stdin: Stdin = io::stdin();
        let mut stdout: Stdout = io::stdout();
        let input = &mut String::new();

        loop {
            input.clear();
            print!("Data dir path for '{config_file_purpose}': ");
            stdout.flush().expect("Flushing output unsuccessful.");
            stdin
                .read_line(input)
                .expect("Reading from input unsuccessful.");

            data_dir = PathBuf::from(String::clone(input).trim_end());

            if data_dir.exists() {
                let mut file = match File::create(config_file_path.clone()) {
                    Err(why) => {
                        panic!(
                            "Couldn't open config file for {} ({})\n{}",
                            config_file_purpose,
                            config_file_path.clone().display(),
                            why
                        );
                    }
                    Ok(file) => file,
                };

                let data_dir_str: &str = match data_dir.to_str() {
                    None => {
                        panic!(
                            "Couldn't convert to string: '{}'",
                            data_dir.clone().display()
                        );
                    }
                    Some(data_dir_str) => data_dir_str,
                };

                if let Err(why) = file.write_all(data_dir_str.as_bytes()) {
                    panic!(
                        "Couldn't write to config file for {} ({})\n{}",
                        config_file_purpose,
                        config_file_path.clone().display(),
                        why
                    );
                }

                break;
            }
        }
    }

    let data_dir_display: Display = data_dir.display();
    logging::info(format!(
        "Loaded data dir for '{config_file_purpose}': '{data_dir_display}'"
    ));

    return data_dir;
}

pub(crate) fn load_data_dirs() -> (PathBuf, PathBuf, PathBuf) {
    let config_dir: PathBuf = load_dir();

    let data_dir_todo: PathBuf =
        load_data_dir_single(&config_dir, "data-dir-path-todo.txt", "ToDo");
    let data_dir_todo_output: PathBuf =
        load_data_dir_single(&config_dir, "data-dir-path-todo-output.txt", "ToDo output");
    let data_dir_finances: PathBuf =
        load_data_dir_single(&config_dir, "data-dir-path-finances.txt", "finances");

    return (data_dir_todo, data_dir_todo_output, data_dir_finances);
}
