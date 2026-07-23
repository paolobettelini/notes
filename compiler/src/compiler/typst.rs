use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) const TYPST_FOLDER: &str = "typst";

pub(super) fn compile_typst(file: &Path, tempdir: &Path) -> Option<PathBuf> {
    let filename = file.file_name()?.to_str()?;
    let pdf = tempdir.join(file.file_stem()?).with_extension("pdf");

    log::info!("Compiling Typst file: {}", filename);

    let out = Command::new("typst")
        .current_dir(file.parent()?)
        .arg("compile")
        .arg(file)
        .arg(&pdf)
        .output()
        .expect("failed to execute typst");

    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        log::error!("Failed to compile file: {}", stderr);
        return None;
    }
    log::debug!("{}", stderr);

    Some(pdf)
}
