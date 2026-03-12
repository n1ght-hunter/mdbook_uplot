# Area Charts

Area charts are line charts with a filled region below the line.

## Basic area chart

```uplot
{
  "type": "area",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  "datasets": [
    { "label": "Revenue", "data": [30, 45, 38, 52, 48, 60], "color": "#2ca02c" }
  ],
  "axes": { "x": "Month", "y": "Revenue ($k)" }
}
```

## Stacked look with multiple series

```uplot
{
  "type": "area",
  "title": "Resource Usage",
  "labels": [1, 2, 3, 4, 5, 6, 7, 8],
  "datasets": [
    { "label": "CPU", "data": [30, 45, 55, 40, 60, 50, 70, 65], "color": "#1f77b4" },
    { "label": "Memory", "data": [50, 55, 60, 58, 65, 62, 68, 72], "color": "#ff7f0e" },
    { "label": "Disk I/O", "data": [10, 15, 20, 12, 18, 22, 25, 20], "color": "#d62728" }
  ],
  "axes": { "x": "Hour", "y": "%" }
}
```

## Custom fill opacity

By default the fill is 30% opacity of the line color. Override with a custom `fill` value:

```uplot
{
  "type": "area",
  "labels": [1, 2, 3, 4, 5, 6],
  "datasets": [
    { "label": "Signal", "data": [5, 12, 8, 15, 10, 18], "color": "#9467bd", "fill": "rgba(148,103,189,0.5)" }
  ]
}
```

## Dataset options

| Field | Type | Description |
|-------|------|-------------|
| `color` | string | Line/stroke color (auto-assigned if omitted) |
| `width` | number | Line width in pixels (default: 2) |
| `fill` | string | Fill color (default: line color at 30% opacity) |
| `dash` | array | Dash pattern, e.g. `[5, 3]` |
| `points` | boolean | Show data point markers |
