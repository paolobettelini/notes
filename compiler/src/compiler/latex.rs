use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub(crate) const LATEX_FOLDER: &str = "latex";
const SEARCH_FOLDER: &str = "packages"; // relative to LATEX_FOLDER

pub(super) fn compile_latex(file: &Path, tempdir: &Path) -> Option<PathBuf> {
    let filename = file.file_name()?.to_str()?;
    log::info!("Compiling LaTeX file: {}", &filename);

    let search_path = format!("search-path={}", SEARCH_FOLDER);

    let out = Command::new("tectonic")
        .current_dir(file.parent().unwrap())
        .arg(file)
        .arg("-Z")
        .arg(search_path)
        .arg("--outdir")
        .arg(tempdir)
        .output()
        .expect("failed to execute tectonic");

    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        log::error!("Failed to compile file: {}", stderr);
        return None;
    }
    log::debug!("{}", stderr);

    Some(tempdir.join(file.file_stem()?).with_extension("pdf"))
}
