use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn query_all_files(path: &PathBuf, containing: &Option<String>) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if let Some(value) = containing {
                    if file_contains_string(&path, &value).unwrap_or(false) {
                        result.push(path);
                    }
                } else {
                    result.push(path);
                }
            }
        }
    }

    result
}

pub fn execute_query(
    path: &PathBuf,
    query: &str,
    regex: bool,
    ignore_case: bool,
    containing: &Option<String>,
) -> Vec<PathBuf> {
    let regex_option = if regex {
        let reg = Regex::new(&query).unwrap_or_else(|_| {
            log::error!("The provided regex is invalid.");
            std::process::exit(1);
        });

        Some(reg)
    } else {
        None
    };

    let mut result = Vec::new();

    // Read the directory
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = if let Ok(name) = entry.file_name().into_string() {
                    name
                } else {
                    log::warn!("File has invalid name: {:?}.", &entry.file_name());
                    continue;
                };

                let mut matched_file = None;

                if let Some(ref reg) = regex_option {
                    // Regex matching
                    if reg.is_match(&file_name) {
                        matched_file = Some(entry.path());
                    }
                } else if ignore_case {
                    // Case insensitive matching
                    let file_name_lower = file_name.to_lowercase();
                    let query_lower = query.to_lowercase();
                    if file_name_lower.contains(&query_lower) {
                        matched_file = Some(entry.path());
                    }
                } else if file_name.contains(&query) {
                    // Case sensitive matching
                    matched_file = Some(entry.path());
                }

                if let Some(path) = matched_file {
                    // Check if content contains given value, if any
                    if let Some(value) = containing {
                        if file_contains_string(&path, &value).unwrap_or(false) {
                            result.push(path);
                        }
                    } else {
                        result.push(path);
                    }
                }
            }
        }
    }

    result
}

pub fn get_notes_path(env: &str) -> PathBuf {
    match std::env::var(env) {
        Err(_) => {
            // The variable `NOTES_PATH` is not set, so assume that the command is being
            // issued from the notes directory directly.
            let current_dir = std::env::current_dir().unwrap_or_else(|_| {
                log::error!(
                    "Could not retrieve current directory. Try setting the `NOTES_PATH` variable."
                );
                std::process::exit(1);
            });

            current_dir
        }
        Ok(v) => PathBuf::from(v),
    }
}

pub fn run_python_script(current_dir: &Path, script_path: &Path, folder_path: &Path) {
    let res = Command::new("python")
        .current_dir(current_dir)
        .arg(script_path)
        .arg(folder_path)
        .output();

    match res {
        Ok(output) => {
            if !output.status.success() {
                log::error!("Script execution failed");
                log::error!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            log::error!("Script execution failed: {}", e);
        }
    }
}

fn file_contains_string(path: &Path, search_string: &str) -> std::io::Result<bool> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.contains(search_string))
}
