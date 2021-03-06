/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::settings::Settings;

use chrono::prelude::*;
use chrono::DateTime;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::path::{Path, PathBuf};

pub fn get_arg_matches() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("(Re)Initialize configuration file")
                .arg(
                    Arg::with_name("source_dir")
                        .help("Source directory")
                        .index(1)
                        .required(true)
                        .value_name("SOURCE"),
                )
                .arg(
                    Arg::with_name("destination_dir")
                        .help("Destination directory")
                        .index(2)
                        .required(true)
                        .value_name("DESTINATION"),
                )
                .arg(
                    Arg::with_name("use_date_pattern")
                        .short("d")
                        .long("use-date-pattern")
                        .help("Use date pattern")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("date_pattern")
                        .short("p")
                        .long("date-pattern")
                        .help("Date subdir pattern")
                        .takes_value(true)
                        .default_value("%Y-%m-%d"), // 2020-01-01
                ),
        )
        .subcommand(SubCommand::with_name("sort").about(
            "Sorting source directory to destination (config file should be initialized first!)",
        ))
        .get_matches()
}

pub fn create_dir(path: &PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(()) => println!("{} dir created successfully!", &path.display()),
            Err(e) => panic!("Error {}", e),
        }
    } else if !path.is_dir() {
        panic!(
            "{} already exists but is not a directory, exiting...",
            &path.display()
        );
    }
}

pub fn create_dirs(dirs: Vec<&PathBuf>) {
    for directory in dirs {
        create_dir(directory);
    }
}

pub fn get_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            files.push(entry.path().to_path_buf())
        }
    }

    files
}

pub fn get_destination_dir(settings: &Settings, file: &Path, destination: &String) -> PathBuf {
    if settings.use_date_pattern {
        let metadata = std::fs::metadata(file);
        let modify_date = DateTime::<Utc>::from(metadata.unwrap().modified().unwrap());
        let date_folder = modify_date.format(&settings.date_pattern).to_string();

        return settings.destination.join(&date_folder).join(destination);
    } else {
        return settings.destination.join(destination);
    }
}

pub fn move_file(file: &Path, destination_dir: &PathBuf, destination_file: &PathBuf) {
    create_dir(destination_dir);

    match std::fs::rename(&file, &destination_file) {
        Ok(_o) => println!(
            "Successfully moved {} to {}",
            &file.display(),
            &destination_dir.display()
        ),
        Err(e) => panic!("Error {}", e),
    }
}
