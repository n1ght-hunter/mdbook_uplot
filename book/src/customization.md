# Customization

## Hide legend

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28], "color": "#1f77b4" }
  ],
  "legend": { "show": false }
}
```

Use `"legend": { "live": true }` to show live values at the cursor position instead of the default static legend.

## Hide tooltip

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28], "color": "#1f77b4" }
  ],
  "tooltip": false
}
```

## Disable crosshair

Remove the vertical hover line:

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28], "color": "#1f77b4" }
  ],
  "opts": { "cursor": { "x": false, "y": false } }
}
```

## Disable hover points

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28], "color": "#1f77b4" }
  ],
  "opts": { "cursor": { "points": { "show": false } } }
}
```

## Enable drag-to-zoom

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  "datasets": [
    { "label": "Value", "data": [5, 12, 8, 22, 15, 30, 18, 25, 10, 35], "color": "#2ca02c" }
  ],
  "opts": { "cursor": { "drag": { "x": true, "y": true } } }
}
```

Double-click to reset zoom.

## Custom axis range

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Value", "data": [10, 25, 18, 32, 28], "color": "#d62728" }
  ],
  "opts": { "scales": { "y": { "range": [0, 50] } } }
}
```

## Custom height

```uplot
{
  "type": "bar",
  "labels": ["A", "B", "C"],
  "datasets": [
    { "label": "Value", "data": [10, 20, 15] }
  ],
  "height": 200
}
```

## Combining options

All fields can be combined:

```uplot
{
  "type": "area",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May"],
  "datasets": [
    { "label": "Revenue", "data": [30, 45, 38, 52, 48], "color": "#9467bd" }
  ],
  "legend": { "show": false },
  "tooltip": false,
  "height": 250,
  "opts": {
    "cursor": { "x": false, "points": { "show": false } }
  }
}
```

## Value formatting

Use `format` to control how values appear in tooltips and y-axis ticks:

```uplot
{
  "type": "line",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  "datasets": [
    { "label": "Revenue", "data": [12.5, 18.3, 15.7, 22.1, 19.8, 25.4], "color": "#2ca02c" }
  ],
  "format": { "prefix": "$", "suffix": "k", "decimals": 1 }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `format.decimals` | number | Decimal places (default: 2) |
| `format.prefix` | string | Prepended to values (e.g. `"$"`) |
| `format.suffix` | string | Appended to values (e.g. `"%"`, `"°C"`) |

Per-dataset `format` overrides the global one:

```uplot
{
  "type": "bar",
  "labels": ["US", "EU", "Asia"],
  "datasets": [
    { "label": "Revenue", "data": [42, 38, 55], "color": "#1f77b4", "format": { "prefix": "$", "suffix": "M", "decimals": 0 } },
    { "label": "Growth", "data": [12.3, 8.7, 15.1], "color": "#ff7f0e", "format": { "suffix": "%", "decimals": 1 } }
  ]
}
```

## Show all series in tooltip

By default the tooltip shows only the nearest series. Use `"tooltip": { "all": true }` to show all series at the cursor position:

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5, 6],
  "datasets": [
    { "label": "CPU", "data": [30, 45, 55, 40, 60, 50], "color": "#1f77b4" },
    { "label": "Memory", "data": [50, 55, 60, 58, 65, 62], "color": "#ff7f0e" },
    { "label": "Disk", "data": [10, 15, 20, 12, 18, 22], "color": "#d62728" }
  ],
  "tooltip": { "all": true },
  "format": { "suffix": "%", "decimals": 0 }
}
```

## Code display

By default every chart shows **Chart | Code** tabs. Use the `code` field to change where and how the code appears:

### Hide code entirely

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Clean", "data": [10, 25, 18, 32, 28], "color": "#17becf" }
  ],
  "code": false
}
```

### Code below the chart

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Below", "data": [10, 25, 18, 32, 28], "color": "#2ca02c" }
  ],
  "code": { "position": "below", "collapsible": false }
}
```

### Collapsible below (closed by default)

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Collapsed", "data": [10, 25, 18, 32, 28], "color": "#ff7f0e" }
  ],
  "code": { "position": "below" }
}
```

### Collapsible below (open by default)

```uplot
{
  "type": "bar",
  "labels": ["A", "B", "C"],
  "datasets": [
    { "label": "Value", "data": [10, 20, 15] }
  ],
  "code": { "position": "below", "open": true }
}
```

### Code above the chart

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Above", "data": [10, 25, 18, 32, 28], "color": "#d62728" }
  ],
  "code": { "position": "above", "open": true }
}
```

### Side by side

```uplot
{
  "type": "line",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May"],
  "datasets": [
    { "label": "Users", "data": [120, 210, 180, 340, 290], "color": "#9467bd" }
  ],
  "code": { "position": "side" },
  "format": { "decimals": 0 }
}
```

On narrow screens the side-by-side layout stacks vertically.

### Options reference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `code` | `false` | — | Hide code entirely |
| `code.position` | `"below"` \| `"above"` \| `"side"` | tabs | Where to place the code. Omit for Chart \| Code tabs |
| `code.collapsible` | boolean | `true` | Wrap in a `<details>` toggle (for above/below) |
| `code.open` | boolean | `false` | Start expanded (when collapsible) |

## Editable playground

Set `"editable": true` to let readers edit the JSON and see the chart update live. Try changing the data or colors below:

```uplot
{
  "type": "line",
  "editable": true,
  "labels": [1, 2, 3, 4, 5, 6],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28, 35], "color": "#1f77b4" },
    { "label": "Series B", "data": [5, 15, 22, 20, 30, 25], "color": "#ff7f0e" }
  ],
  "tooltip": { "all": true }
}
```

## Advanced: `opts` reference

The `opts` field is deep-merged into the uPlot configuration before chart creation. Any valid [uPlot option](https://github.com/leeoniya/uPlot/blob/master/dist/uPlot.d.ts) can be passed here, including `cursor`, `scales`, `axes`, `select`, and more.
