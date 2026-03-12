# Bar Charts

## Basic bar chart

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

Renders as:

```uplot
{
  "labels": ["a", "b", "c"],
  "datasets": [
    { "label": "Series 1", "data": [10, 20, 30], "color": "#e74c3c" }
  ],
  "axes": { "x": "Category", "y": "Value" }
}
```

## Multiple series

````markdown
```uplot
{
  "title": "Quarterly Sales",
  "labels": ["Q1", "Q2", "Q3", "Q4"],
  "datasets": [
    { "label": "Product A", "data": [50, 80, 45, 90], "color": "#2ca02c" },
    { "label": "Product B", "data": [30, 60, 70, 40], "color": "#1f77b4" },
    { "label": "Product C", "data": [20, 35, 55, 65], "color": "#ff7f0e" }
  ],
  "axes": { "x": "Quarter", "y": "Revenue ($k)" }
}
```
````

Renders as:

```uplot
{
  "title": "Quarterly Sales",
  "labels": ["Q1", "Q2", "Q3", "Q4"],
  "datasets": [
    { "label": "Product A", "data": [50, 80, 45, 90], "color": "#2ca02c" },
    { "label": "Product B", "data": [30, 60, 70, 40], "color": "#1f77b4" },
    { "label": "Product C", "data": [20, 35, 55, 65], "color": "#ff7f0e" }
  ],
  "axes": { "x": "Quarter", "y": "Revenue ($k)" }
}
```

## Options

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Chart title (optional) |
| `labels` | array | X-axis labels |
| `datasets` | array | One or more data series |
| `datasets[].label` | string | Series name |
| `datasets[].data` | array | Numeric values |
| `datasets[].color` | string | CSS color (optional, auto-assigned if omitted) |
| `axes.x` | string | X-axis label (optional) |
| `axes.y` | string | Y-axis label (optional) |
| `height` | number | Chart height in pixels (default: 350) |
