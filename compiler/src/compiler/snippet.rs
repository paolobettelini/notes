use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use stellar_database::ClientHandler;
use tokio::fs::create_dir_all;
use crate::utils::run_python_script;

const BUILD_SCRIPT_FILENAME: &str = "build.py";

pub async fn compile_snippet<'a>(
    folder: &'a Path,
    data: &PathBuf,
    db_client: &ClientHandler,
) -> bool {
    // NANNOU:
    //    # compile
    //    wasm-pack build --release
    //
    //    cd website
    //    npm install # TODO: only if necessary
    //    npm run build
    //
    //
    //    # generate snippet folder
    //    rm "dist/index.js"
    //    mv "dist/* "$target_folder"
    //    rm -tf "dist/"

    // TODO: Do not compile resources/ or packages/
    // maybe put an empty compilation script there
    
    let filename = match folder.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            log::error!("Failed to get the filename from the provided path.");
            return false;
        }
    };

    log::info!("Compiling snippet: {}", &filename);

    let mut target_folder = data.join(crate::SNIPPETS_FOLDER);
    target_folder.push(&filename);
    
    // Delete the target folder if it exists
    if target_folder.exists() {
        if let Err(e) = fs::remove_dir_all(&target_folder) {
            log::error!(
                "Failed to remove existing directory {}: {}",
                target_folder.display(),
                e
            );
            return false;
        }
    }

    // Create the target folder
    if let Err(e) = create_dir_all(&target_folder).await {
        log::error!(
            "Failed to create directory {}: {}",
            target_folder.display(),
            e
        );
        return false;
    }

    let python_custom_script = contains_build_script(&folder);

    if let Some(script) = python_custom_script {
       // Run "<BUILD_SCRIPT_FILENAME>" with the target directory as a parameter

        log::info!("Running custom build script for snippet {}", &filename);

        run_python_script(&folder, &script, &target_folder);
    } else {
        // Copy all contents from the source folder to the target folder
        if let Err(e) = copy_recursively(folder, &target_folder) {
            log::error!(
                "Failed to copy contents to {}: {}",
                target_folder.display(),
                e
            );
            return false;
        }
    }

    // Import
    stellar_import::import_snippet_with_client(db_client, folder)
        .await
        .unwrap_or_else(|e| {
            log::error!("Could not import snippet: {}", e);
        });

    true
}

// Helper function to copy all contents of a directory asynchronously
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn contains_build_script<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Attempt to read the directory
    match fs::read_dir(path.as_ref()) {
        Ok(entries) => {
            // Iterate over the directory entries
            for entry in entries {
                if let Ok(entry) = entry {
                    // Check if the file name is "<BUILD_SCRIPT_FILENAME>"
                    if entry.file_name() == BUILD_SCRIPT_FILENAME {
                        return Some(entry.path());
                    }
                }
            }
            None
        },
        Err(_) => None,
    }
}
