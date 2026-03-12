use mdbook_preprocessor::Preprocessor;
use mdbook_uplot::{ASSET_FILES, Uplot};
use toml_edit::{Array, DocumentMut, Item, Table, Value, value};

use std::{env, fs, io, path::Path, process};

fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_writer(io::stderr)
        .init();

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("supports") => {
            let renderer = args.get(2).map(String::as_str).unwrap_or("");
            if matches!(Uplot.supports_renderer(renderer), Ok(true)) {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
        Some("install") => {
            let dir = args.get(2).map(String::as_str).unwrap_or(".");
            handle_install(Path::new(dir));
        }
        _ => {
            if let Err(e) = handle_preprocessing() {
                tracing::error!("{e}");
                process::exit(1);
            }
        }
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

    ensure_assets(&ctx.root);

    let processed = Uplot.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}

fn needs_write(filepath: &Path, content: &str) -> bool {
    match fs::read_to_string(filepath) {
        Ok(existing) => existing != content,
        Err(_) => true,
    }
}

fn ensure_assets(root: &Path) {
    for &(name, content) in ASSET_FILES {
        let filepath = root.join(name);
        if needs_write(&filepath, content) {
            if let Some(parent) = filepath.parent() {
                fs::create_dir_all(parent).expect("can't create asset directory");
            }
            tracing::info!("Writing '{name}'");
            fs::write(&filepath, content).expect("can't write asset file");
        }
    }

    let config_path = root.join("book.toml");
    let toml = fs::read_to_string(&config_path).expect("can't read book.toml");
    let mut doc: DocumentMut = toml.parse().expect("book.toml is not valid TOML");

    let mut changed = false;
    for &(name, _) in ASSET_FILES {
        let ext = name.rsplit('.').next().unwrap_or("js");
        if add_additional_file(&mut doc, ext, name) {
            changed = true;
        }
    }

    if changed {
        fs::write(&config_path, doc.to_string()).expect("can't write book.toml");
        tracing::error!(
            "book.toml was updated with new assets, but you need to re-run 'mdbook build' to pick up the changes"
        );
        process::exit(1);
    }
}

fn handle_install(dir: &Path) -> ! {
    let config = dir.join("book.toml");
    if !config.exists() {
        tracing::error!("Configuration file '{}' not found", config.display());
        process::exit(1);
    }

    let toml = fs::read_to_string(&config).expect("can't read configuration file");
    let mut doc: DocumentMut = toml.parse().expect("configuration is not valid TOML");

    let mut changed = false;

    if !has_preprocessor(&doc) {
        tracing::info!("Adding [preprocessor.uplot] to book.toml");
        add_preprocessor(&mut doc);
        changed = true;
    }

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

    for &(name, content) in ASSET_FILES {
        let filepath = dir.join(name);
        if needs_write(&filepath, content) {
            if let Some(parent) = filepath.parent() {
                fs::create_dir_all(parent).expect("can't create asset directory");
            }
            tracing::info!("Writing '{name}'");
            fs::write(&filepath, content).expect("can't write asset file");
        } else {
            tracing::debug!("'{name}' is up to date, skipping");
        }
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
        return false;
    }

    tracing::info!("Adding '{file}' to '{key}'");
    insert_additional(doc, &key, file);
    true
}

fn get_additional_array<'a>(doc: &'a DocumentMut, key: &str) -> Option<&'a Array> {
    doc.get("output")?.get("html")?.get(key)?.as_array()
}

fn has_file(arr: &Option<&Array>, file: &str) -> bool {
    match arr {
        Some(arr) => arr.iter().any(|v| v.as_str().is_some_and(|s| s == file)),
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
