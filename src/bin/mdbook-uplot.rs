use clap::{Arg, ArgMatches, Command};
use mdbook_preprocessor::Preprocessor;
use mdbook_uplot::{Uplot, ASSET_FILES, HEAD_HBS};
use toml_edit::{value, Array, DocumentMut, Item, Table, Value};

use std::{fs, io, path::PathBuf, process};

fn make_app() -> Command {
    Command::new("mdbook-uplot")
        .version(env!("CARGO_PKG_VERSION"))
        .about("mdbook preprocessor for interactive uPlot charts")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            Command::new("install")
                .arg(
                    Arg::new("dir")
                        .default_value(".")
                        .help("Root directory for the book, should contain book.toml"),
                )
                .about("Install the preprocessor configuration and assets into book.toml"),
        )
}

fn main() {
    tracing_subscriber::fmt::init();

    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else if let Some(sub_args) = matches.subcommand_matches("install") {
        handle_install(sub_args);
    } else if let Err(e) = handle_preprocessing() {
        tracing::error!("{e}");
        process::exit(1);
    }
}

fn handle_preprocessing() -> mdbook_preprocessor::errors::Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook_preprocessor::MDBOOK_VERSION {
        tracing::warn!(
            "mdbook-uplot was built against mdbook {}, but called from {}",
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed = Uplot.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("required argument");

    if let Ok(true) = Uplot.supports_renderer(renderer) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_install(sub_args: &ArgMatches) -> ! {
    let proj_dir = sub_args
        .get_one::<String>("dir")
        .expect("required argument");
    let proj_dir = PathBuf::from(proj_dir);
    let config = proj_dir.join("book.toml");

    if !config.exists() {
        tracing::error!("Configuration file '{}' not found", config.display());
        process::exit(1);
    }

    tracing::info!("Reading {}", config.display());
    let toml = fs::read_to_string(&config).expect("can't read configuration file");
    let mut doc: DocumentMut = toml.parse().expect("configuration is not valid TOML");

    let mut changed = false;

    if !has_preprocessor(&doc) {
        tracing::info!("Adding [preprocessor.uplot] to book.toml");
        add_preprocessor(&mut doc);
        changed = true;
    }

    // Add local asset files to additional-js/additional-css
    for &(name, _) in ASSET_FILES {
        let ext = name.rsplit('.').next().unwrap_or("js");
        if add_additional_file(&mut doc, ext, name) {
            changed = true;
        }
    }

    if changed {
        tracing::info!("Saving configuration to {}", config.display());
        fs::write(&config, doc.to_string()).expect("can't write configuration file");
    }

    // Write asset files to project directory
    for &(name, content) in ASSET_FILES {
        let filepath = proj_dir.join(name);
        if filepath.exists() {
            tracing::debug!("'{}' already exists, skipping", name);
        } else {
            tracing::info!("Writing '{}'", name);
            fs::write(&filepath, content).expect("can't write asset file");
        }
    }

    // Write theme/head.hbs for CDN includes
    let theme_dir = proj_dir.join("theme");
    let head_hbs = theme_dir.join("head.hbs");
    if head_hbs.exists() {
        let existing = fs::read_to_string(&head_hbs).unwrap_or_default();
        if existing.contains("uplot") {
            tracing::debug!("theme/head.hbs already contains uPlot references, skipping");
        } else {
            tracing::info!("Appending uPlot CDN links to theme/head.hbs");
            let mut content = existing;
            content.push_str(HEAD_HBS);
            fs::write(&head_hbs, content).expect("can't write theme/head.hbs");
        }
    } else {
        tracing::info!("Writing theme/head.hbs");
        fs::create_dir_all(&theme_dir).expect("can't create theme directory");
        fs::write(&head_hbs, HEAD_HBS).expect("can't write theme/head.hbs");
    }

    tracing::info!("mdbook-uplot installed. Use ```uplot code blocks in your markdown.");
    process::exit(0);
}

fn has_preprocessor(doc: &DocumentMut) -> bool {
    doc.get("preprocessor")
        .and_then(|p| p.get("uplot"))
        .map(|m| matches!(m, Item::Table(_)))
        .unwrap_or(false)
}

fn add_preprocessor(doc: &mut DocumentMut) {
    let empty_table = Item::Table(Table::default());

    let item = doc
        .as_table_mut()
        .entry("preprocessor")
        .or_insert(empty_table.clone());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("uplot")
        .or_insert(empty_table);
    item["command"] = value("mdbook-uplot");
}

fn add_additional_file(doc: &mut DocumentMut, file_type: &str, file: &str) -> bool {
    let key = format!("additional-{file_type}");

    let existing = get_additional_array(doc, &key);
    if has_file(&existing, file) {
        tracing::debug!("'{}' already in '{}', skipping", file, key);
        return false;
    }

    tracing::info!("Adding '{}' to '{}'", file, key);
    insert_additional(doc, &key, file);
    true
}

fn get_additional_array<'a>(doc: &'a DocumentMut, key: &str) -> Option<&'a Array> {
    doc.get("output")?
        .get("html")?
        .get(key)?
        .as_array()
}

fn has_file(arr: &Option<&Array>, file: &str) -> bool {
    match arr {
        Some(arr) => arr.iter().any(|v| v.as_str().map_or(false, |s| s == file)),
        None => false,
    }
}

fn insert_additional(doc: &mut DocumentMut, key: &str, file: &str) {
    let empty_table = Item::Table(Table::default());
    let empty_array = Item::Value(Value::Array(Array::default()));

    let item = doc
        .as_table_mut()
        .entry("output")
        .or_insert(empty_table.clone());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("html")
        .or_insert(empty_table);
    let array = item
        .as_table_mut()
        .unwrap()
        .entry(key)
        .or_insert(empty_array);
    array
        .as_value_mut()
        .unwrap()
        .as_array_mut()
        .unwrap()
        .push(file);
}
