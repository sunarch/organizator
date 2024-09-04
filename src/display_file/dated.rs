// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
// internal
use crate::dated;
use crate::logging;
use crate::tasks::data::TaskData;

pub(crate) fn print(task_data: &TaskData, data_dir_todo_output: &Path) {
    let output_file_name: &str = "dated.md";
    let output_file_path: PathBuf = data_dir_todo_output.join(output_file_name);
    logging::info(format!(
        "Writing to output file '{}",
        output_file_path.clone().display()
    ));
    let file: File = match File::create(output_file_path.clone()) {
        Err(why) => {
            panic!(
                "Couldn't open output file  '{}'\n{}",
                output_file_path.clone().display(),
                why
            );
        }
        Ok(file) => file,
    };

    dated::print_list(task_data, &output_fn, &mut Some(file));
}

fn output_fn(line: &String, file_option: &mut Option<File>) {
    match file_option {
        None => {
            panic!("No file handle! (dated)");
        }
        Some(file) => {
            if let Err(why) = file.write_all(line.as_bytes()) {
                panic!("Couldn't write to output file\n{}", why);
            }
            if let Err(why) = file.write_all(String::from("\n").as_bytes()) {
                panic!("Couldn't write to output file\n{}", why);
            }
            file.flush().expect("Unable to flush write to output file.");
        }
    }
}
