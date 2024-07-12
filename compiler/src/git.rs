use git2::{Repository, StatusOptions};
use log::{error, info};
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::io::{BufRead, BufReader};

pub fn git_pull_and_get_files(notes_path: &PathBuf) -> Vec<PathBuf> {
    // git pull
    let mut child = Command::new("git")
        .current_dir(notes_path)
        .arg("pull")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute `git pull`");

    // Capture stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => log::info!("{}", line),
                Err(e) => log::error!("Error reading stdout: {}", e),
            }
        }
    }

    // Capture stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => log::error!("{}", line),
                Err(e) => log::error!("Error reading stderr: {}", e),
            }
        }
    }

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        log::error!("Command `git pull` was unsuccessful");
        std::process::exit(1);
    }

    let mut result = vec![];

    // git diff --name-status HEAD@{1}..HEAD
    let output = Command::new("git")
        .current_dir(notes_path)
        .arg("diff")
        .arg("--name-status")
        .arg("HEAD@{1}..HEAD")
        .output()
        .expect("failed to execute `git diff --name-status HEAD@{1}..HEAD`");

    if let Ok(stdout) = String::from_utf8(output.stdout) {
        if stdout.is_empty() {
            return vec![];
        }

        for entry in stdout.split('\n') {
            // Added or modifier
            if entry.starts_with('M') || entry.starts_with('A') {
                let filename = &entry[2..];
                result.push(notes_path.join(filename));
            }

        }
    }

    result
}

/*
pub fn git_pull_and_get_files(notes_path: &PathBuf) -> Vec<PathBuf> {
    // Log the git pull execution
    info!("Executing `git pull` operation");

    // Open the existing repository
    let repo = Repository::open(notes_path).unwrap_or_else(|e| {
        error!("Failed to open repository: {}", e);
        std::process::exit(1);
    });

    // Perform the git pull operation
    let mut remote = repo.find_remote("origin").unwrap_or_else(|e| {
        error!("Failed to find remote `origin`: {}", e);
        std::process::exit(1);
    });

    remote.fetch(&["main"], None, None).unwrap_or_else(|e| {
        error!("Failed to fetch from remote `origin`: {}", e);
        std::process::exit(1);
    });

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap_or_else(|e| {
        error!("Failed to find FETCH_HEAD: {}", e);
        std::process::exit(1);
    });

    let fetch_commit = repo
        .reference_to_annotated_commit(&fetch_head)
        .unwrap_or_else(|e| {
            error!("Failed to convert FETCH_HEAD to commit: {}", e);
            std::process::exit(1);
        });

    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap_or_else(|e| {
        error!("Merge analysis failed: {}", e);
        std::process::exit(1);
    });

    if analysis.0.is_up_to_date() {
        info!("The repository is already up to date.");
        return Vec::new();
    }

    if analysis.0.is_fast_forward() {
        let mut reference = repo.find_reference("refs/heads/main").unwrap_or_else(|e| {
            error!("Failed to find reference `refs/heads/main`: {}", e);
            std::process::exit(1);
        });

        reference
            .set_target(fetch_commit.id(), "Fast-Forward")
            .unwrap_or_else(|e| {
                error!("Failed to set target for reference: {}", e);
                std::process::exit(1);
            });

        repo.set_head("refs/heads/main").unwrap_or_else(|e| {
            error!("Failed to set head to `refs/heads/main`: {}", e);
            std::process::exit(1);
        });

        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .unwrap_or_else(|e| {
                error!("Failed to checkout head: {}", e);
                std::process::exit(1);
            });
    } else {
        error!("Non-fast-forward merges are not supported in this example.");
        std::process::exit(1);
    }

    // Get the status options to retrieve modified and created files
    let mut status_opts = StatusOptions::new();
    status_opts
        .include_untracked(true)
        .recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut status_opts)).unwrap_or_else(|e| {
        error!("Failed to get repository statuses: {}", e);
        std::process::exit(1);
    });

    // Collect paths of modified and created files
    let files: Vec<PathBuf> = statuses
        .iter()
        .filter_map(|entry| {
            let status = entry.status();
            if status.is_wt_new() || status.is_wt_modified() {
                entry.path().map(|p| notes_path.join(p))
            } else {
                None
            }
        })
        .collect();

    files
}*/
