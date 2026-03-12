# Usage

In your markdown, use a fenced code block with the `uplot` language tag.

## Inline data

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

## External data file

Instead of inlining data, you can reference a JSON file:

````markdown
```uplot
{ "data": "path/to/data.json" }
```
````

Paths are resolved relative to the chapter's source directory. The referenced file should contain the same JSON structure (with `labels`, `datasets`, etc.).
