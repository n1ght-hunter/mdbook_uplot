/// Generic uPlot chart renderer for mdbook-uplot.
///
/// JSON schema (Chart.js-inspired):
/// {
///   "title": "Chart Title",
///   "labels": ["x1", "x2", "x3"],
///   "datasets": [
///     { "label": "series A", "data": [1.0, 2.0, 3.0], "color": "#2ca02c" },
///     { "label": "series B", "data": [1.5, 2.5, 3.5], "color": "#1f77b4" }
///   ],
///   "axes": { "x": "Parameter", "y": "Time (ns)" }
/// }

(function () {
  "use strict";

  document.addEventListener("DOMContentLoaded", function () {
    document.querySelectorAll(".uplot-chart").forEach(initChart);
  });

  function initChart(container) {
    const dataEl = container.querySelector("script.uplot-data");
    if (!dataEl) return;

    let chartJson;
    try {
      chartJson = JSON.parse(dataEl.textContent);
    } catch (e) {
      container.innerHTML = "<p>Failed to parse chart data: " + e.message + "</p>";
      return;
    }

    if (chartJson.labels && chartJson.datasets) {
      renderBarChart(container, chartJson);
    }
  }

  function renderBarChart(container, json) {
    const labels = json.labels || [];
    const datasets = json.datasets || [];
    const title = json.title || "";
    const xLabel = json.axes?.x || "";
    const yLabel = json.axes?.y || "";

    if (labels.length === 0 || datasets.length === 0) {
      container.innerHTML = "<p><em>No data available.</em></p>";
      return;
    }

    // Build uPlot columnar data: [labels, ...datasetValues]
    const uData = [labels];
    const uSeries = [{ label: xLabel || "x" }];

    for (const ds of datasets) {
      uData.push(ds.data || labels.map(() => null));
      uSeries.push({
        label: ds.label || "series",
        fill: ds.color || nextColor(uSeries.length - 1),
        stroke: ds.color || nextColor(uSeries.length - 1),
        width: 0,
      });
    }

    const theme = detectTheme();
    const height = json.height || 350;

    const opts = {
      title: title,
      width: container.clientWidth || 800,
      height: height,
      legend: { live: false },
      axes: [
        {
          stroke: theme.text,
          label: xLabel,
        },
        {
          stroke: theme.text,
          label: yLabel,
          ticks: { stroke: theme.grid },
          grid: { stroke: theme.grid },
        },
      ],
      series: uSeries,
      plugins: [
        seriesBarsPlugin({
          ori: 0,
          dir: 1,
          radius: 0.3,
        }),
        tooltipPlugin(json),
      ],
    };

    const chart = new uPlot(opts, uData, container);

    const ro = new ResizeObserver(entries => {
      for (const entry of entries) {
        const w = entry.contentRect.width;
        if (w > 0) chart.setSize({ width: w, height: height });
      }
    });
    ro.observe(container);
  }

  function tooltipPlugin(json) {
    let tooltipEl;
    const suffix = json.axes?.y ? " " + json.axes.y.split("(").pop()?.replace(")", "") : "";

    function init(u) {
      tooltipEl = document.createElement("div");
      tooltipEl.className = "uplot-tooltip";
      tooltipEl.style.display = "none";
      u.over.appendChild(tooltipEl);
      u.over.addEventListener("mouseleave", () => {
        tooltipEl.style.display = "none";
      });
    }

    function setCursor(u) {
      let found = false;
      for (let i = 1; i < u.series.length; i++) {
        const idx = u.cursor.idxs?.[i];
        if (idx != null) {
          const s = u.series[i];
          const val = u.data[i][idx];
          const label = u.data[0][idx];
          if (val != null && s.show) {
            const color = s.fill || s.stroke;
            tooltipEl.innerHTML =
              "<strong>" + esc(String(label)) + "</strong><br>" +
              '<span style="color:' + color + '">■</span> ' +
              esc(s.label) + ": " + val.toFixed(2);
            tooltipEl.style.display = "block";
            tooltipEl.style.left = (u.cursor.left + 15) + "px";
            tooltipEl.style.top = (u.cursor.top - 10) + "px";

            const overRect = u.over.getBoundingClientRect();
            const tipRect = tooltipEl.getBoundingClientRect();
            if (tipRect.right > overRect.right)
              tooltipEl.style.left = (u.cursor.left - tipRect.width - 10) + "px";
            if (tipRect.bottom > overRect.bottom)
              tooltipEl.style.top = (u.cursor.top - tipRect.height - 10) + "px";

            found = true;
            break;
          }
        }
      }
      if (!found) tooltipEl.style.display = "none";
    }

    return { hooks: { init, setCursor } };
  }

  function detectTheme() {
    const el = document.documentElement;
    const isDark = el.classList.contains("coal")
      || el.classList.contains("navy")
      || el.classList.contains("ayu");
    return {
      text: isDark ? "#c8c9db" : "#333",
      grid: isDark ? "rgba(255,255,255,0.1)" : "rgba(0,0,0,0.1)",
    };
  }

  const PALETTE = [
    "#2ca02c", "#1f77b4", "#ff7f0e", "#d62728", "#9467bd",
    "#17becf", "#e377c2", "#bcbd22", "#98df8a", "#888888",
  ];

  function nextColor(i) { return PALETTE[i % PALETTE.length]; }
  function esc(s) { return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); }
})();
