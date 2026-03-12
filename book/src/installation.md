# Installation

## Using `cargo-binstall`

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) already:

```sh
cargo binstall mdbook-uplot
```

## Manually

Binary releases are available on the [Releases page](https://github.com/n1ght-hunter/mdbook_uplot/releases).
Download the appropriate binary for your platform, make it executable, and add it to your PATH.

## Setup

Run the install command pointing at your book directory:

```sh
mdbook-uplot install /path/to/book
```

The `install` command updates `book.toml` (preprocessor config, additional-js/css) and writes asset files and CDN links to `theme/head.hbs`.
