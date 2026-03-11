# mdbook-uplot

An [mdbook](https://rust-lang.github.io/mdBook/) preprocessor for embedding interactive [uPlot](https://github.com/leeoniya/uPlot) charts in your book.

## Installation

### Using `cargo-binstall`

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) already:

```
cargo binstall mdbook-uplot
```

### Manually

Binary releases are available on the [Releases page](https://github.com/n1ght-hunter/mdbook_uplot/releases).
Download the appropriate binary for your platform, make it executable, and add it to your PATH.


## Setup

```sh
mdbook-uplot install /path/to/book
```

The `install` command updates `book.toml` (preprocessor config, additional-js/css) and writes asset files and CDN links to `theme/head.hbs`.

## Usage


In your markdown, use a fenced code block with the `uplot` language tag.

### Inline data

````markdown
```uplot
{
  "labels": ["a", "b", "c"],
  "datasets": [
    { "label": "Series 1", "data": [10, 20, 30], "color": "#e74c3c" }
  ],
  "axes": { "x": "Category", "y": "Value" }
}
```
````

### External data file

````markdown
```uplot
{ "data": "path/to/data.json" }
```
````

Paths are resolved relative to the chapter's source directory.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
Licensed under either of
- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
