# Usage

Use fenced code blocks with the `uplot` language tag to create charts.

````markdown
```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4],
  "datasets": [
    { "label": "Series 1", "data": [10, 20, 15, 30] }
  ]
}
```
````

## Chart types

Set the `type` field to choose a chart style:

| Type | Description |
|------|-------------|
| `"bar"` | Bar chart (default if `type` is omitted) |
| `"line"` | Line chart |
| `"area"` | Area chart (line with filled region) |
| `"scatter"` | Scatter plot (individual points) |

See the sub-pages for examples and type-specific options.

## Common fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Chart type (default: `"bar"`) |
| `title` | string | Chart title (optional) |
| `labels` | array | X-axis values |
| `datasets` | array | One or more data series |
| `datasets[].label` | string | Series name |
| `datasets[].data` | array | Numeric values |
| `datasets[].color` | string | CSS color (auto-assigned if omitted) |
| `axes.x` | string | X-axis label (optional) |
| `axes.y` | string | Y-axis label (optional) |
| `height` | number | Chart height in pixels (default: 350) |

## Custom uPlot options

The `opts` field lets you deep-merge arbitrary options into the uPlot
configuration. This gives access to uPlot's full API without needing custom JS:

````markdown
```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [{ "label": "Value", "data": [10, 25, 18, 32, 28] }],
  "opts": {
    "scales": { "y": { "range": [0, 50] } }
  }
}
```
````

## External data files

Instead of inlining data, reference a JSON file:

````markdown
```uplot
{ "data": "path/to/data.json" }
```
````

Paths are resolved relative to the chapter's source directory. The referenced
file should contain the same JSON structure (with `type`, `labels`, `datasets`, etc.).
