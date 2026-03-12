(function () {
  "use strict";

  function init() {
    var charts = document.querySelectorAll(".uplot-chart");
    if (charts.length === 0) return;
    charts.forEach(initChart);
  }

  function initChart(container) {
    var dataEl = container.querySelector("script.uplot-data");
    if (!dataEl) return;

    var chartJson;
    try {
      chartJson = JSON.parse(dataEl.textContent);
    } catch (e) {
      container.innerHTML = "<p>Failed to parse chart data: " + e.message + "</p>";
      return;
    }

    if (!chartJson.labels || !chartJson.datasets) return;

    var type = chartJson.type || "bar";
    switch (type) {
      case "line":    renderChart(container, chartJson, "line"); break;
      case "area":    renderChart(container, chartJson, "area"); break;
      case "scatter": renderChart(container, chartJson, "scatter"); break;
      case "bar":
      default:        renderChart(container, chartJson, "bar"); break;
    }
  }

  function renderChart(container, json, type) {
    var labels = json.labels || [];
    var datasets = json.datasets || [];
    var title = json.title || "";
    var xLabel = (json.axes && json.axes.x) || "";
    var yLabel = (json.axes && json.axes.y) || "";

    if (labels.length === 0 || datasets.length === 0) {
      container.innerHTML = "<p><em>No data available.</em></p>";
      return;
    }

    var hasStringLabels = typeof labels[0] === "string";
    var xValues = hasStringLabels
      ? labels.map(function (_, i) { return i; })
      : labels;

    var uData = type === "bar" ? [labels] : [xValues];
    var uSeries = [{ label: xLabel || "x" }];

    for (var i = 0; i < datasets.length; i++) {
      var ds = datasets[i];
      var color = ds.color || nextColor(i);
      uData.push(ds.data || labels.map(function () { return null; }));
      uSeries.push(buildSeries(ds, color, type, i));
    }

    var theme = detectTheme();
    var height = json.height || 350;

    var xAxis = { stroke: theme.text, label: xLabel };
    if (type !== "bar" && hasStringLabels) {
      xAxis.values = function (u, splits) {
        return splits.map(function (v) {
          var idx = Math.round(v);
          return idx >= 0 && idx < labels.length ? labels[idx] : "";
        });
      };
      xAxis.space = function () { return 80; };
    }

    var opts = {
      title: title,
      width: container.clientWidth || 800,
      height: height,
      legend: json.legend || { live: false },
      scales: type !== "bar" ? { x: { time: false } } : {},
      axes: [
        xAxis,
        {
          stroke: theme.text,
          label: yLabel,
          ticks: { stroke: theme.grid },
          grid: { stroke: theme.grid },
        },
      ],
      series: uSeries,
      plugins: json.tooltip === false ? [] : [tooltipPlugin(json, type === "bar" ? labels : null)],
    };

    if (type === "bar") {
      opts.plugins.unshift(seriesBarsPlugin({ ori: 0, dir: 1, radius: 0.3 }));
    }

    if (type === "scatter") {
      opts.cursor = opts.cursor || {};
      opts.cursor.points = { show: false };
    }

    if (json.opts) {
      opts = deepMerge(opts, json.opts);
    }

    var chart = new uPlot(opts, uData, container);

    var ro = new ResizeObserver(function (entries) {
      for (var j = 0; j < entries.length; j++) {
        var w = entries[j].contentRect.width;
        if (w > 0) chart.setSize({ width: w, height: height });
      }
    });
    ro.observe(container);
  }

  function buildSeries(ds, color, type, index) {
    var s = { label: ds.label || "series" };

    switch (type) {
      case "line":
        s.stroke = color;
        s.width = ds.width || 2;
        s.fill = ds.fill || null;
        if (ds.dash) s.dash = ds.dash;
        if (ds.points) s.points = { show: true, size: 4 };
        break;

      case "area":
        s.stroke = color;
        s.width = ds.width || 2;
        s.fill = ds.fill || colorAlpha(color, 0.3);
        if (ds.dash) s.dash = ds.dash;
        if (ds.points) s.points = { show: true, size: 4 };
        break;

      case "scatter":
        s.stroke = color;
        s.fill = color;
        s.paths = function () { return null; };
        s.points = {
          show: true,
          size: ds.pointSize || 6,
          fill: color,
        };
        break;

      case "bar":
      default:
        s.fill = color;
        s.stroke = color;
        s.width = 0;
        break;
    }

    return s;
  }

  function colorAlpha(hex, alpha) {
    if (hex.charAt(0) !== "#" || hex.length < 7) return hex;
    var r = parseInt(hex.slice(1, 3), 16);
    var g = parseInt(hex.slice(3, 5), 16);
    var b = parseInt(hex.slice(5, 7), 16);
    return "rgba(" + r + "," + g + "," + b + "," + alpha + ")";
  }

  function deepMerge(target, source) {
    var out = {};
    for (var k in target) {
      if (target.hasOwnProperty(k)) out[k] = target[k];
    }
    for (var k in source) {
      if (!source.hasOwnProperty(k)) continue;
      if (
        out[k] && typeof out[k] === "object" && !Array.isArray(out[k]) &&
        source[k] && typeof source[k] === "object" && !Array.isArray(source[k])
      ) {
        out[k] = deepMerge(out[k], source[k]);
      } else {
        out[k] = source[k];
      }
    }
    return out;
  }

  function tooltipPlugin(json, barLabels) {
    var tooltipEl;

    function initHook(u) {
      tooltipEl = document.createElement("div");
      tooltipEl.className = "uplot-tooltip";
      tooltipEl.style.display = "none";
      u.over.appendChild(tooltipEl);
      u.over.addEventListener("mouseleave", function () {
        tooltipEl.style.display = "none";
      });
    }

    function setCursor(u) {
      var found = false;
      for (var i = 1; i < u.series.length; i++) {
        var idx = u.cursor.idxs && u.cursor.idxs[i];
        if (idx != null) {
          var s = u.series[i];
          var val = u.data[i][idx];
          var rawX = u.data[0][idx];
          var label = barLabels ? barLabels[idx] : (json.labels && json.labels[Math.round(rawX)] != null ? json.labels[Math.round(rawX)] : rawX);
          if (val != null && s.show) {
            var color = s.fill || s.stroke;
            tooltipEl.innerHTML =
              "<strong>" + esc(String(label)) + "</strong><br>" +
              '<span style="color:' + color + '">■</span> ' +
              esc(s.label) + ": " + val.toFixed(2);
            tooltipEl.style.display = "block";
            tooltipEl.style.left = (u.cursor.left + 15) + "px";
            tooltipEl.style.top = (u.cursor.top - 10) + "px";

            var overRect = u.over.getBoundingClientRect();
            var tipRect = tooltipEl.getBoundingClientRect();
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

    return { hooks: { init: initHook, setCursor: setCursor } };
  }

  function detectTheme() {
    var el = document.documentElement;
    var isDark = el.classList.contains("coal")
      || el.classList.contains("navy")
      || el.classList.contains("ayu");
    return {
      text: isDark ? "#c8c9db" : "#333",
      grid: isDark ? "rgba(255,255,255,0.1)" : "rgba(0,0,0,0.1)",
    };
  }

  var PALETTE = [
    "#2ca02c", "#1f77b4", "#ff7f0e", "#d62728", "#9467bd",
    "#17becf", "#e377c2", "#bcbd22", "#98df8a", "#888888",
  ];

  function nextColor(i) { return PALETTE[i % PALETTE.length]; }
  function esc(s) { return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
