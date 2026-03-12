# mdbook-uplot

An [mdbook](https://rust-lang.github.io/mdBook/) preprocessor for embedding interactive [uPlot](https://github.com/leeoniya/uPlot) charts in your book.

Write chart data as fenced `` ```uplot `` code blocks in your markdown and mdbook-uplot renders them as interactive charts.

## Quick example

```uplot
{
  "type": "line",
  "title": "Monthly Revenue",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  "datasets": [
    { "label": "2025", "data": [30, 45, 38, 52, 48, 60], "color": "#1f77b4" },
    { "label": "2026", "data": [35, 50, 42, 58, 55, 68], "color": "#2ca02c" }
  ],
  "axes": { "x": "Month", "y": "Revenue ($k)" }
}
```

## Features

- **Bar, line, area, and scatter** chart types
- **Interactive** — tooltips, hover crosshair, drag-to-zoom
- **Customizable** — hide legend, disable tooltips, set axis ranges, pass any uPlot option via `opts`
- **Zero runtime dependencies** — all assets are bundled in the binary
- **Dark theme support** — automatically adapts to mdbook's coal, navy, and ayu themes
