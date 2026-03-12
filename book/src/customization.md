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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

## Disable crosshair

Remove the vertical hover line:

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28], "color": "#1f77b4" }
  ],
  "opts": { "cursor": { "x": false } }
}
```

<details>
<summary>Show code</summary>

````markdown
```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Series A", "data": [10, 25, 18, 32, 28], "color": "#1f77b4" }
  ],
  "opts": { "cursor": { "x": false } }
}
```
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

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

<details>
<summary>Show code</summary>

````markdown
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
````

</details>

## Advanced: `opts` reference

The `opts` field is deep-merged into the uPlot configuration before chart creation. Any valid [uPlot option](https://github.com/leeoniya/uPlot/blob/master/dist/uPlot.d.ts) can be passed here, including `cursor`, `scales`, `axes`, `select`, and more.
