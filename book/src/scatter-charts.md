# Scatter Charts

Scatter charts display individual data points without connecting lines.

## Basic scatter chart

```uplot
{
  "type": "scatter",
  "labels": [1, 2, 3, 4, 5, 6, 7, 8],
  "datasets": [
    { "label": "Measurements", "data": [3, 7, 2, 8, 5, 9, 4, 6], "color": "#d62728" }
  ],
  "axes": { "x": "Sample", "y": "Value" }
}
```

<details>
<summary>Show code</summary>

````markdown
```uplot
{
  "type": "scatter",
  "labels": [1, 2, 3, 4, 5, 6, 7, 8],
  "datasets": [
    { "label": "Measurements", "data": [3, 7, 2, 8, 5, 9, 4, 6], "color": "#d62728" }
  ],
  "axes": { "x": "Sample", "y": "Value" }
}
```
````

</details>

## Multiple series

```uplot
{
  "type": "scatter",
  "title": "Test Results",
  "labels": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  "datasets": [
    { "label": "Group A", "data": [5, 8, 3, 9, 6, 7, 4, 8, 5, 9], "color": "#1f77b4" },
    { "label": "Group B", "data": [3, 5, 7, 4, 8, 6, 9, 3, 7, 5], "color": "#ff7f0e" }
  ],
  "axes": { "x": "Trial", "y": "Score" }
}
```

<details>
<summary>Show code</summary>

````markdown
```uplot
{
  "type": "scatter",
  "title": "Test Results",
  "labels": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  "datasets": [
    { "label": "Group A", "data": [5, 8, 3, 9, 6, 7, 4, 8, 5, 9], "color": "#1f77b4" },
    { "label": "Group B", "data": [3, 5, 7, 4, 8, 6, 9, 3, 7, 5], "color": "#ff7f0e" }
  ],
  "axes": { "x": "Trial", "y": "Score" }
}
```
````

</details>

## Custom point size

```uplot
{
  "type": "scatter",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Large", "data": [2, 5, 3, 7, 4], "color": "#2ca02c", "pointSize": 10 },
    { "label": "Small", "data": [6, 3, 8, 2, 6], "color": "#9467bd", "pointSize": 4 }
  ]
}
```

<details>
<summary>Show code</summary>

````markdown
```uplot
{
  "type": "scatter",
  "labels": [1, 2, 3, 4, 5],
  "datasets": [
    { "label": "Large", "data": [2, 5, 3, 7, 4], "color": "#2ca02c", "pointSize": 10 },
    { "label": "Small", "data": [6, 3, 8, 2, 6], "color": "#9467bd", "pointSize": 4 }
  ]
}
```
````

</details>

## Dataset options

| Field | Type | Description |
|-------|------|-------------|
| `color` | string | Point color (auto-assigned if omitted) |
| `pointSize` | number | Point diameter in pixels (default: 6) |
