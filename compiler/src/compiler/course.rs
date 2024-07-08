use std::fs;
use std::path::Path;
use std::path::PathBuf;
use stellar_database::ClientHandler;
use tokio::fs::create_dir_all;

pub async fn compile_course<'a>(file: &'a Path, data: &PathBuf, db_client: &ClientHandler) -> bool {
    let filename = match file.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            log::error!("Failed to get the filename from the provided path.");
            return false;
        }
    };

    let id = filename.replace(".json", "");
    log::info!("Compiling course: {}", &id);

    let mut folder = data.join(crate::COURSES_FOLDER);
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
            log::error!(
                "Failed to remove existing file {}: {}",
                target_file.display(),
                e
            );
            return false;
        }
    }

    if let Err(e) = fs::copy(file, &target_file) {
        log::error!("Failed to copy file to {}: {}", target_file.display(), e);
        return false;
    }

    // Import
    stellar_import::import_course_with_client(db_client, file)
        .await
        .unwrap_or_else(|e| {
            log::error!("Could not import course: {}", e);
        });

    true
}
