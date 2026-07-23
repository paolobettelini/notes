use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use stellar_database::ClientHandler;

use super::{
    latex::{LATEX_FOLDER, compile_latex},
    typst::{TYPST_FOLDER, compile_typst},
};

type CompileFn = fn(&Path, &Path) -> Option<PathBuf>;

#[derive(Clone, Copy)]
struct Margins {
    top: f64,
    bottom: f64,
}

#[derive(Clone, Copy)]
struct DocumentFormat {
    name: &'static str,
    folder: &'static str,
    compile: CompileFn,
    margins: Margins,
}

fn document_formats() -> &'static HashMap<&'static str, DocumentFormat> {
    static FORMATS: OnceLock<HashMap<&'static str, DocumentFormat>> = OnceLock::new();

    FORMATS.get_or_init(|| {
        HashMap::from([
            (
                "tex",
                DocumentFormat {
                    name: "LaTeX",
                    folder: LATEX_FOLDER,
                    compile: compile_latex,
                    margins: Margins {
                        top: 20.0,
                        bottom: -9.5,
                    },
                },
            ),
            (
                "typ",
                DocumentFormat {
                    name: "Typst",
                    folder: TYPST_FOLDER,
                    compile: compile_typst,
                    margins: Margins {
                        top: 32.0,
                        bottom: -4.0,
                    },
                },
            ),
        ])
    })
}

pub fn is_document_folder(folder: &str) -> bool {
    document_formats()
        .values()
        .any(|format| format.folder == folder)
}

pub async fn compile_document(
    file: &Path,
    data: &Path,
    db_client: &ClientHandler,
    tempdir: &Path,
) -> (bool, u32) {
    let Some(extension) = file.extension().and_then(|extension| extension.to_str()) else {
        log::warn!("File {} has no extension.", file.display());
        return (false, 0);
    };

    let Some(format) = document_formats().get(extension) else {
        log::warn!(
            "Unsupported document extension \".{}\" for file {}.",
            extension,
            file.display()
        );
        return (false, 0);
    };

    let Some(pdf) = (format.compile)(file, tempdir) else {
        return (false, 0);
    };

    generate_pdf(&pdf, data, db_client, format).await
}

async fn generate_pdf(
    pdf: &PathBuf,
    data: &Path,
    db_client: &ClientHandler,
    format: &DocumentFormat,
) -> (bool, u32) {
    log::info!(
        "Generating snippets and pages from {} PDF: {}",
        format.name,
        pdf.display()
    );

    stellar_pdfformat::generate_snippets(
        pdf,
        data,
        Some(db_client),
        format.margins.top,
        format.margins.bottom,
        None,
        None,
    )
    .await
    .unwrap_or((false, 0))
}
