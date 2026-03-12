# Usage

Use fenced code blocks with the `uplot` language tag to create charts.

````markdown
```uplot
{
  "labels": ["a", "b", "c"],
  "datasets": [
    { "label": "Series 1", "data": [10, 20, 30] }
  ]
}
```
````

The JSON inside the code block defines the chart. See the chart type pages for
the full schema and examples.

## External data files

Instead of inlining data, you can reference a JSON file:

````markdown
```uplot
{ "data": "path/to/data.json" }
```
````

Paths are resolved relative to the chapter's source directory. The referenced
file should contain the same JSON structure (with `labels`, `datasets`, etc.).
