/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortPattern {
    pub extensions: Vec<String>,
    pub mime_types: Vec<String>,
    pub destination: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub sources: Vec<PathBuf>,
    pub destination: PathBuf,
    pub use_date_pattern: bool,
    pub date_pattern: String,
    pub sort_patterns: Vec<SortPattern>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sources: vec![],
            destination: PathBuf::new(),
            use_date_pattern: false,
            date_pattern: String::new(),
            sort_patterns: vec![
                // Archives
                SortPattern {
                    extensions: vec![
                        String::from("7z"),
                        String::from("gz"),
                        String::from("rar"),
                        String::from("tar"),
                        String::from("tgz"),
                        String::from("xz"),
                        String::from("zip"),
                        String::from("zst"),
                    ],
                    mime_types: vec![],
                    destination: String::from("archives"),
                },
                // Audio
                SortPattern {
                    extensions: vec![
                        String::from("flac"),
                        String::from("mp3"),
                        String::from("ogg"),
                        String::from("opus"),
                        String::from("wav"),
                    ],
                    mime_types: vec![],
                    destination: String::from("audio"),
                },
                // Binary
                SortPattern {
                    extensions: vec![String::from("exe"), String::from("bin")],
                    mime_types: vec![
                        String::from("application/x-pie-executable"),
                        String::from("application/x-sharedlib"),
                    ],
                    destination: String::from("binary"),
                },
                // Images
                SortPattern {
                    extensions: vec![
                        String::from("gif"),
                        String::from("jpeg"),
                        String::from("jpg"),
                        String::from("png"),
                        String::from("tif"),
                    ],
                    mime_types: vec![],
                    destination: String::from("images"),
                },
                SortPattern {
                    extensions: vec![
                        String::from("avi"),
                        String::from("mkv"),
                        String::from("mp4"),
                    ],
                    mime_types: vec![],
                    destination: String::from("videos"),
                },
                // Documents
                SortPattern {
                    extensions: vec![
                        String::from("csv"),
                        String::from("djvu"),
                        String::from("doc"),
                        String::from("docx"),
                        String::from("epub"),
                        String::from("odt"),
                        String::from("pdf"),
                        String::from("ppt"),
                        String::from("pptx"),
                        String::from("txt"),
                    ],
                    mime_types: vec![],
                    destination: String::from("docs"),
                },
                // Packages
                SortPattern {
                    extensions: vec![String::from("rpm"), String::from("spec")],
                    mime_types: vec![],
                    destination: String::from("rpm-packages"),
                },
                SortPattern {
                    extensions: vec![String::from("deb")],
                    mime_types: vec![],
                    destination: String::from("debian-packages"),
                },
                SortPattern {
                    extensions: vec![String::from("apk"), String::from("apkx")],
                    mime_types: vec![],
                    destination: String::from("apks"),
                },
                // Other
                SortPattern {
                    extensions: vec![String::from("torrent")],
                    mime_types: vec![],
                    destination: String::from("torrents"),
                },
                SortPattern {
                    extensions: vec![String::from("jar")],
                    mime_types: vec![],
                    destination: String::from("jars"),
                },
                SortPattern {
                    extensions: vec![String::from("xml")],
                    mime_types: vec![],
                    destination: String::from("xml"),
                },
                SortPattern {
                    extensions: vec![String::from("img")],
                    mime_types: vec![],
                    destination: String::from("raw"),
                },
                SortPattern {
                    extensions: vec![
                        String::from("eot"),
                        String::from("ttf"),
                        String::from("woff"),
                        String::from("woff2"),
                    ],
                    mime_types: vec![],
                    destination: String::from("fonts"),
                },
                SortPattern {
                    extensions: vec![String::from("ovpn")],
                    mime_types: vec![],
                    destination: String::from("openvpn-profiles"),
                },
                SortPattern {
                    extensions: vec![String::from("pcap")],
                    mime_types: vec![],
                    destination: String::from("captured-packages"),
                },
                SortPattern {
                    extensions: vec![String::from("vsix")],
                    mime_types: vec![],
                    destination: String::from("vscode-extensions"),
                },
            ],
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = Settings::get_settings_path();

        if let Ok(file) = fs::File::open(&path) {
            match serde_yaml::from_reader(file) {
                Ok(s) => return s,
                Err(e) => {
                    println!("Failed to parse setting file! Fallback to default. {}", e);
                    // Rename the corrupted settings file
                    let mut new_path = path.to_owned();
                    new_path.pop();
                    new_path.push("settings.json.invalid");
                    if let Err(err) = std::fs::rename(path, new_path) {
                        println!("Failed to rename settings file. {}", err);
                    }
                }
            }
        }
        // This is reached if either:
        // - The file can't be opened (presumably it doesn't exist)
        // - Or there was an error parsing the file
        let default_settings = Self::default();
        default_settings
    }

    pub fn add_source(&mut self, source: PathBuf) -> &mut Self {
        self.sources.push(source);
        self
    }

    pub fn destination(&mut self, destination: PathBuf) -> &mut Self {
        self.destination = destination;
        self
    }

    pub fn use_date_pattern(&mut self, use_date_pattern: bool) -> &mut Self {
        self.use_date_pattern = use_date_pattern;
        self
    }

    pub fn date_pattern(&mut self, date_pattern: String) -> &mut Self {
        self.date_pattern = date_pattern;
        self
    }

    pub fn save_to_file_warn(&self) {
        if let Err(err) = self.save_to_file() {
            panic!("Failed to save settings: {:?}", err);
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let path = Settings::get_settings_path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut config_file = fs::File::create(path)?;

        let s: &str = &serde_yaml::to_string(self).unwrap();
        config_file.write_all(s.as_bytes()).unwrap();

        Ok(())
    }

    pub fn backup_old_config(&self) -> &Self {
        let settings_file = Settings::get_settings_path();
        let settings_file_old = format!("{}.old", &settings_file.display());
        if Path::new(&settings_file).exists() {
            match std::fs::rename(&settings_file, &settings_file_old) {
                Ok(_o) => {
                    println!(
                        "Moved old settings file to {} successfully",
                        &settings_file_old
                    );
                }
                Err(e) => panic!("Error {}", e),
            }
        }

        self
    }

    pub fn get_settings_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "elxreno", "filesorter")
            .expect("System's $HOME directory path not found!");

        proj_dirs
            .config_dir()
            .join("settings")
            .with_extension("yaml")
    }
}
