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

    if (chartJson.labels && chartJson.datasets) {
      renderBarChart(container, chartJson);
    }
  }

  function renderBarChart(container, json) {
    var labels = json.labels || [];
    var datasets = json.datasets || [];
    var title = json.title || "";
    var xLabel = (json.axes && json.axes.x) || "";
    var yLabel = (json.axes && json.axes.y) || "";

    if (labels.length === 0 || datasets.length === 0) {
      container.innerHTML = "<p><em>No data available.</em></p>";
      return;
    }

    var uData = [labels];
    var uSeries = [{ label: xLabel || "x" }];

    for (var i = 0; i < datasets.length; i++) {
      var ds = datasets[i];
      uData.push(ds.data || labels.map(function () { return null; }));
      uSeries.push({
        label: ds.label || "series",
        fill: ds.color || nextColor(uSeries.length - 1),
        stroke: ds.color || nextColor(uSeries.length - 1),
        width: 0,
      });
    }

    var theme = detectTheme();
    var height = json.height || 350;

    var opts = {
      title: title,
      width: container.clientWidth || 800,
      height: height,
      legend: { live: false },
      axes: [
        { stroke: theme.text, label: xLabel },
        {
          stroke: theme.text,
          label: yLabel,
          ticks: { stroke: theme.grid },
          grid: { stroke: theme.grid },
        },
      ],
      series: uSeries,
      plugins: [
        seriesBarsPlugin({ ori: 0, dir: 1, radius: 0.3 }),
        tooltipPlugin(json),
      ],
    };

    var chart = new uPlot(opts, uData, container);

    var ro = new ResizeObserver(function (entries) {
      for (var j = 0; j < entries.length; j++) {
        var w = entries[j].contentRect.width;
        if (w > 0) chart.setSize({ width: w, height: height });
      }
    });
    ro.observe(container);
  }

  function tooltipPlugin(json) {
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
          var label = u.data[0][idx];
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
