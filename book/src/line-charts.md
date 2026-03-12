# Line Charts

## Basic line chart

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5, 6],
  "datasets": [
    { "label": "Temperature", "data": [22, 24, 19, 28, 25, 30], "color": "#1f77b4" }
  ],
  "axes": { "x": "Day", "y": "°C" }
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
    { "label": "Temperature", "data": [22, 24, 19, 28, 25, 30], "color": "#1f77b4" }
  ],
  "axes": { "x": "Day", "y": "°C" }
}
```
````

</details>

## Multiple series

```uplot
{
  "type": "line",
  "title": "Monthly Visitors",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  "datasets": [
    { "label": "Site A", "data": [120, 150, 180, 200, 170, 220], "color": "#2ca02c" },
    { "label": "Site B", "data": [80, 90, 110, 130, 160, 140], "color": "#d62728" }
  ],
  "axes": { "x": "Month", "y": "Visitors (k)" }
}
```

<details>
<summary>Show code</summary>

````markdown
```uplot
{
  "type": "line",
  "title": "Monthly Visitors",
  "labels": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  "datasets": [
    { "label": "Site A", "data": [120, 150, 180, 200, 170, 220], "color": "#2ca02c" },
    { "label": "Site B", "data": [80, 90, 110, 130, 160, 140], "color": "#d62728" }
  ],
  "axes": { "x": "Month", "y": "Visitors (k)" }
}
```
````

</details>

## With points and dashes

Set `"points": true` to show data points, and `"dash": [5, 3]` for dashed lines:

```uplot
{
  "type": "line",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Actual", "data": [10, 25, 18, 32, 28], "color": "#1f77b4", "points": true },
    { "label": "Target", "data": [15, 20, 25, 30, 35], "color": "#ff7f0e", "dash": [5, 3] }
  ]
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
    { "label": "Actual", "data": [10, 25, 18, 32, 28], "color": "#1f77b4", "points": true },
    { "label": "Target", "data": [15, 20, 25, 30, 35], "color": "#ff7f0e", "dash": [5, 3] }
  ]
}
```
````

</details>

## Dataset options

| Field | Type | Description |
|-------|------|-------------|
| `color` | string | Line color (auto-assigned if omitted) |
| `width` | number | Line width in pixels (default: 2) |
| `dash` | array | Dash pattern, e.g. `[5, 3]` |
| `points` | boolean | Show data point markers |
| `fill` | string | Fill color below the line (creates an area effect) |
