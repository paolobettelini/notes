use std::path::{Path, PathBuf};
use std::fs;
use stellar_database::ClientHandler;
use tokio::fs::{create_dir_all, read_dir};
use tokio::io::AsyncWriteExt;
use std::io;

pub async fn compile_snippet<'a>(folder: &'a Path, data: &PathBuf, db_client: &ClientHandler) -> bool {
    // TODO: check for build.py
    // NANNOU:
    //    snippet_name=$(basename "$1")
    //    
    //    # cleanup old versions
    //    if [ -e "$1/website/dist/index.js" ]; then
    //        rm "$1/website/dist/index.js"
    //    fi
    //    if [ -e "$data_folder/snippets/$snippet_name" ]; then
    //        rm -rf "$data_folder/snippets/$snippet_name*"
    //    fi
    //
    //    cd "$1"
    //    # compile using shared folder to cache shared packages
    //    wasm-pack build
    //
    //    cd website
    //    npm install # --dry-run --quiet || npm install # run install only if necessary
    //    npm run build
    //
    //    cd "$WORKING_DIR"
    //    mkdir -p "$data_folder/snippets/$snippet_name"
    //
    //    # generate snippet folder
    //    rm "$1/website/dist/index.js"
    //    mv "$1"/website/dist/* "$data_folder/snippets/$snippet_name"
    //
    // TODO: Do not compile resources/ or packages/
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
            log::error!("Failed to remove existing directory {}: {}", target_folder.display(), e);
            return false;
        }
    }

    // Create the target folder
    if let Err(e) = create_dir_all(&target_folder).await {
        log::error!("Failed to create directory {}: {}", target_folder.display(), e);
        return false;
    }

    // Copy all contents from the source folder to the target folder
    if let Err(e) = copy_recursively(folder, &target_folder) {
        log::error!("Failed to copy contents to {}: {}", target_folder.display(), e);
        return false;
    }

    log::info!("Snippet compiled successfully.");
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
