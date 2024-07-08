use std::path::PathBuf;
use std::path::Path;
use std::fs;
use tokio::fs::create_dir_all;
use stellar_database::ClientHandler;

pub async fn compile_universe<'a>(file: &'a Path, data: &PathBuf, db_client: &ClientHandler) -> bool {
    let filename = match file.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            log::error!("Failed to get the filename from the provided path.");
            return false;
        }
    };

    let id = filename.replace(".json", "");
    log::info!("Compiling universe: {}", &id);

    let mut folder = data.join(crate::UNIVERSES_FOLDER);
    folder.push(&id);

    if !folder.exists() {
        if let Err(e) = create_dir_all(&folder).await {
            log::error!("Failed to create directory {}: {}", folder.display(), e);
            return false;
        }
    }

    let target_file = folder.join(filename);

    if target_file.exists() {
        if let Err(e) = fs::remove_file(&target_file) {
            log::error!("Failed to remove existing file {}: {}", target_file.display(), e);
            return false;
        }
    }

    if let Err(e) = fs::copy(file, &target_file) {
        log::error!("Failed to copy file to {}: {}", target_file.display(), e);
        return false;
    }

    true
}
