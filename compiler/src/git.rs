use git2::{Repository, StatusOptions};
use std::path::{PathBuf, Path};
use log::{info, error};

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

    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap_or_else(|e| {
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

        reference.set_target(fetch_commit.id(), "Fast-Forward").unwrap_or_else(|e| {
            error!("Failed to set target for reference: {}", e);
            std::process::exit(1);
        });

        repo.set_head("refs/heads/main").unwrap_or_else(|e| {
            error!("Failed to set head to `refs/heads/main`: {}", e);
            std::process::exit(1);
        });

        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).unwrap_or_else(|e| {
            error!("Failed to checkout head: {}", e);
            std::process::exit(1);
        });
    } else {
        error!("Non-fast-forward merges are not supported in this example.");
        std::process::exit(1);
    }

    // Get the status options to retrieve modified and created files
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true).recurse_untracked_dirs(true);

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
}
