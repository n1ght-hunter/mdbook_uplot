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

## Advanced: `opts` reference

The `opts` field is deep-merged into the uPlot configuration before chart creation. Any valid [uPlot option](https://github.com/leeoniya/uPlot/blob/master/dist/uPlot.d.ts) can be passed here, including `cursor`, `scales`, `axes`, `select`, and more.
