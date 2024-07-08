use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use stellar_database::ClientHandler;

const SEARCH_FOLDER: &str = "packages"; // relative to LATEX_FOLDER

pub async fn compile_latex<'a>(
    file: &'a Path,
    data: &PathBuf,
    db_client: &ClientHandler,
    tempdir: &Path,
) -> (bool, u32) {
    let filename = file.file_name().unwrap().to_str().unwrap();

    if !filename.contains(".tex") {
        log::warn!("File {} does not have \".tex\" extension.", &filename);
        return (false, 0);
    }

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

    // println!("{}", String::from_utf8(out.stdout).unwrap());

    if let Ok(stdout) = String::from_utf8(out.stderr) {
        if stdout.contains("error: ") {
            log::error!("Failed to compile file: {}", stdout);
            return (false, 0);
        } else {
            log::debug!("{}", stdout);
        }
    }

    log::info!("Generating snippets and pages from: {}", &filename);

    let pdf = tempdir.join(filename.replace(".tex", ".pdf"));

    let top_offset = -20.0;
    let bottom_offset = 9.5;
    let left_margin = None;
    let right_margin = None;

    let res = stellar_pdfformat::generate_snippets(
        &pdf,
        data,
        Some(db_client),
        top_offset,
        bottom_offset,
        left_margin,
        right_margin,
    )
    .await;

    if let Ok(stats) = res {
        stats
    } else {
        (false, 0) // no page imported, 0 snippets imported
    }
}
