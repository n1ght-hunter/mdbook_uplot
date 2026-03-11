use maud::{html, PreEscaped};
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::Fenced, Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub const BARS_JS: &str = include_str!("../assets/uplot-bars.js");
pub const CHARTS_JS: &str = include_str!("../assets/uplot-charts.js");
pub const CHARTS_CSS: &str = include_str!("../assets/uplot-charts.css");

pub const ASSET_FILES: &[(&str, &str)] = &[
    ("uplot-bars.js", BARS_JS),
    ("uplot-charts.js", CHARTS_JS),
    ("uplot-charts.css", CHARTS_CSS),
];

pub const HEAD_HBS: &str = concat!(
    "<link rel=\"stylesheet\" href=\"https://unpkg.com/uplot@1.6.32/dist/uPlot.min.css\">\n",
    "<script src=\"https://unpkg.com/uplot@1.6.32/dist/uPlot.iife.min.js\"></script>\n",
);

pub struct Uplot;

impl Preprocessor for Uplot {
    fn name(&self) -> &str {
        "uplot"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let src_dir = ctx.root.join(&ctx.config.book.src);

        book.for_each_chapter_mut(|chapter| {
            if chapter.content.contains("```uplot") {
                let chapter_dir = chapter
                    .path
                    .as_ref()
                    .and_then(|p| p.parent())
                    .map(|p| src_dir.join(p))
                    .unwrap_or_else(|| src_dir.clone());

                chapter.content = add_uplot(&chapter.content, &chapter_dir);
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}

#[derive(Deserialize)]
struct ChartConfig {
    data: Option<String>,
    labels: Option<Vec<serde_json::Value>>,
}

fn add_uplot(content: &str, chapter_dir: &Path) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut in_uplot = false;
    let mut code_span = 0..0;
    let mut start_new_code_span = true;
    let mut uplot_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (event, span) in events.into_offset_iter() {
        if let Event::Start(Tag::CodeBlock(Fenced(lang))) = &event {
            if lang.as_ref() == "uplot" {
                in_uplot = true;
                start_new_code_span = true;
            }
            continue;
        }

        if !in_uplot {
            continue;
        }

        if let Event::Text(_) = &event {
            if start_new_code_span {
                code_span = span;
                start_new_code_span = false;
            } else {
                code_span = code_span.start..span.end;
            }
            continue;
        }

        if let Event::End(TagEnd::CodeBlock) = &event {
            in_uplot = false;
            start_new_code_span = true;

            let json_text = &content[code_span.clone()];
            let replacement = match resolve_chart_data(json_text, chapter_dir) {
                Ok(data_json) => render_chart_html(&data_json),
                Err(msg) => render_chart_error(&msg),
            };
            uplot_blocks.push((span, replacement));
        }
    }

    let mut result = content.to_string();
    for (span, block) in uplot_blocks.iter().rev() {
        let pre = &result[..span.start];
        let post = &result[span.end..];
        result = format!("{pre}\n{block}{post}");
    }

    result
}

fn resolve_chart_data(block: &str, chapter_dir: &Path) -> std::result::Result<String, String> {
    let config: ChartConfig =
        serde_json::from_str(block).map_err(|e| format!("invalid JSON: {e}"))?;

    if config.labels.is_some() {
        Ok(block.to_string())
    } else if let Some(path) = &config.data {
        let resolved = resolve_data_path(path, chapter_dir);
        std::fs::read_to_string(&resolved)
            .map_err(|_| format!("could not read data file: {}", resolved.display()))
    } else {
        Err("JSON must have either \"labels\" (inline data) or \"data\" (file path)".to_string())
    }
}

fn render_chart_html(data_json: &str) -> String {
    html! {
        div class="uplot-chart" {
            script type="application/json" class="uplot-data" {
                (PreEscaped(data_json))
            }
        }
    }
    .into_string()
        + "\n"
}

fn render_chart_error(msg: &str) -> String {
    html! {
        div class="uplot-chart uplot-error" {
            p { "Chart error: " (msg) }
        }
    }
    .into_string()
        + "\n"
}

fn resolve_data_path(data_path: &str, chapter_dir: &Path) -> PathBuf {
    let p = Path::new(data_path);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        chapter_dir.join(p)
    }
}

