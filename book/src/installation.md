# Installation

## Shell (macOS/Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/n1ght-hunter/mdbook_uplot/releases/latest/download/mdbook_uplot-installer.sh | sh
```

## PowerShell (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/n1ght-hunter/mdbook_uplot/releases/latest/download/mdbook_uplot-installer.ps1 | iex"
```

## cargo-binstall

```sh
cargo binstall mdbook_uplot
```

## From crates.io

```sh
cargo install mdbook_uplot
```

## Setup

Add the preprocessor to your `book.toml`:

```toml
[preprocessor.uplot]
```

Then either run `mdbook build` twice (the first run writes the asset files and updates
`book.toml`, the second picks up the changes), or add the following to `book.toml` manually:

```toml
[output.html]
additional-js = ["assets/uplot/uplot.min.js", "assets/uplot/uplot-bars.js", "assets/uplot/uplot-init.js"]
additional-css = ["assets/uplot/uplot.min.css", "assets/uplot/uplot-charts.css"]
```

Alternatively, run the install command to do everything in one step:

```sh
mdbook-uplot install /path/to/book
```
