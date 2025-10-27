use args::*;
use clap::Parser;
use std::collections::HashSet;
use std::path::PathBuf;
use stellar_database::ClientHandler;

mod args;
mod compiler;
mod git;
mod utils;

const LOG_ENV: &str = "RUST_LOG";
const NOTES_PATH_ENV: &str = "NOTES_PATH";
const DATA_PATH_ENV: &str = "NOTES_DATA_PATH";
const MONGODB_URL_ENV: &str = "MONGO_CONNECTION_URL";

const SOURCE_FOLDER: &str = "source";
const DATA_FOLDER: &str = "data";
const LATEX_FOLDER: &str = "latex";
const SNIPPETS_FOLDER: &str = "snippets";
const PAGES_FOLDER: &str = "pages";
const COURSES_FOLDER: &str = "courses";
const UNIVERSES_FOLDER: &str = "universes";

#[tokio::main]
async fn main() {
    /* === Init === */

    if std::env::var(LOG_ENV).is_err() {
        std::env::set_var(LOG_ENV, "info");
    }
    env_logger::init();

    let args = App::parse();
    args.validate();

    /* === Logic === */

    let notes_path = utils::get_notes_path(&NOTES_PATH_ENV);

    let data_path = if let Ok(v) = std::env::var(DATA_PATH_ENV) {
        PathBuf::from(v)
    } else {
        // Assume the data folder is in the notes folder
        notes_path.join(DATA_FOLDER)
    };

    let db_client = if let Ok(v) = std::env::var(MONGODB_URL_ENV) {
        let res = ClientHandler::new(&v).await;

        match res {
            Ok(client) => client,
            Err(e) => {
                log::error!("Could not connect to MongoDB server: {e:?}");
                std::process::exit(1);
            }
        }
    } else {
        log::error!("Variable `MONGO_CONNECTION_URL` is not set.");
        std::process::exit(1);
    };

    let search_all_folders =
        !(args.latex || args.snippets || args.pages || args.courses || args.universes);
    let search_latex = args.latex || search_all_folders;
    let search_snippets = args.snippets || search_all_folders;
    let search_pages = args.pages || search_all_folders;
    let search_courses = args.courses || search_all_folders;
    let search_universes = args.universes || search_all_folders;

    let mut processed_pdfs_count = 0;
    let mut imported_snippets_count = 0;
    let mut imported_pages_count = 0;
    let mut imported_courses_count = 0;
    let mut imported_universes_count = 0;

    if args.git {
        // Query git

        let files = git::git_pull_and_get_files(&notes_path);

        compile_generic_files(
            &data_path,
            &files,
            &db_client,
            &mut processed_pdfs_count,
            &mut imported_snippets_count,
            &mut imported_pages_count,
            &mut imported_courses_count,
            &mut imported_universes_count,
        )
        .await;
    } else {
        // Query the folder(s)

        macro_rules! get_files {
            ($folder:ident) => {
                if let Some(inputs) = &args.inputs {
                    // If input(s) is specified, query the input(s)
                    let mut files = vec![];

                    for input in inputs {
                        let res = utils::execute_query(
                            &$folder,
                            input,
                            args.regex,
                            args.ignore_case,
                            &args.containing,
                        );
                        files.extend(res);
                    }

                    files
                } else {
                    // If no input is specified, query for every file
                    utils::query_all_files(&$folder, &args.containing)
                }
            };
        }

        let source_path = notes_path.join(SOURCE_FOLDER);

        if search_latex {
            let folder = source_path.join(LATEX_FOLDER);
            let files = get_files!(folder);

            let dir = tempdir::TempDir::new("stellar").unwrap();
            for file in files {
                let (page, snippets) =
                    compiler::compile_latex(&file, &data_path, &db_client, dir.path()).await;

                imported_pages_count += page as u32;
                imported_snippets_count += snippets;
                processed_pdfs_count += 1;
            }
        }

        if search_snippets {
            let folder = source_path.join(SNIPPETS_FOLDER);
            let files = get_files!(folder);

            for file in files {
                let res = compiler::compile_snippet(&file, &data_path, &db_client).await;
                imported_snippets_count += res as u32;
            }
        }

        if search_pages {
            let folder = source_path.join(PAGES_FOLDER);
            let files = get_files!(folder);

            for file in files {
                let res = compiler::compile_page(&file, &data_path, &db_client).await;
                imported_pages_count += res as u32;
            }
        }

        if search_courses {
            let folder = source_path.join(COURSES_FOLDER);
            let files = get_files!(folder);

            for file in files {
                let res = compiler::compile_course(&file, &data_path, &db_client).await;
                imported_courses_count += res as u32;
            }
        }

        if search_universes {
            let folder = source_path.join(UNIVERSES_FOLDER);
            let files = get_files!(folder);

            for file in files {
                let res = compiler::compile_universe(&file, &data_path, &db_client).await;
                imported_universes_count += res as u32;
            }
        }
    }

    log::info!("=== [STATS] ===");
    if args.git || search_latex {
        log::info!("Processed PDFs: {}", processed_pdfs_count);
    }
    if args.git || search_snippets || search_latex {
        log::info!("Imported snippets: {}", imported_snippets_count);
    }
    if args.git || search_pages || search_latex {
        log::info!("Imported pages: {}", imported_pages_count);
    }
    if args.git || search_courses {
        log::info!("Imported courses: {}", imported_courses_count);
    }
    if args.git || search_universes {
        log::info!("Imported universe: {}", imported_universes_count);
    }
}

async fn compile_generic_files(
    data_path: &PathBuf,
    files: &[PathBuf],
    db_client: &ClientHandler,
    processed_pdfs_count: &mut u32,
    imported_snippets_count: &mut u32,
    imported_pages_count: &mut u32,
    imported_courses_count: &mut u32,
    imported_universes_count: &mut u32,
) {
    // Used to avoid compiling same thing multiple times
    let mut compiled_snippets = HashSet::new();
    // When pages will have their own folder, do the same

    for file in files {
        let mut current_path = file.as_path();

        let dir = tempdir::TempDir::new("stellar").unwrap();

        while let Some(parent) = current_path.parent() {
            if let Some(folder_name) = parent.file_name().and_then(|name| name.to_str()) {
                // parent.to_path_buf()
                if folder_name == LATEX_FOLDER {
                    if compiled_snippets.contains(current_path) {
                        current_path = parent;
                        continue;
                    }

                    let (page, snippets) =
                        compiler::compile_latex(&current_path, data_path, db_client, dir.path())
                            .await;

                    *imported_pages_count += page as u32;
                    *imported_snippets_count += snippets;
                    *processed_pdfs_count += 1;

                    compiled_snippets.insert(current_path);
                }
                if folder_name == SNIPPETS_FOLDER {
                    let res =
                        compiler::compile_snippet(&current_path, &data_path, &db_client).await;
                    *imported_snippets_count += res as u32;
                }
                if folder_name == PAGES_FOLDER {
                    let res = compiler::compile_page(&current_path, &data_path, &db_client).await;
                    *imported_pages_count += res as u32;
                }
                if folder_name == COURSES_FOLDER {
                    let res = compiler::compile_course(&current_path, &data_path, &db_client).await;
                    *imported_courses_count += res as u32;
                }
                if folder_name == UNIVERSES_FOLDER {
                    let res =
                        compiler::compile_universe(&current_path, &data_path, &db_client).await;
                    *imported_universes_count += res as u32;
                }
            }
            current_path = parent;
        }
    }
}
