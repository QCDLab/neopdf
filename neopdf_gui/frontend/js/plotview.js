// Plot view: render an mpld3 figure spec into the plot container.

/**
 * Render an mpld3 figure specification (JSON string) into the plot container.
 *
 * D3 v5 and mpld3 are already loaded in the page <head>, so we just need
 * to parse the spec and call mpld3.draw_figure().
 *
 * @param {string} specJson - The mpld3 figure spec as a JSON string.
 */
function embedPlot(specJson) {
  const container = document.getElementById("plot-container");
  container.innerHTML = "";

  // Create a unique div for mpld3 to draw into
  const figId = "fig_" + Date.now();
  const div = document.createElement("div");
  div.id = figId;
  container.appendChild(div);

  const spec = JSON.parse(specJson);
  mpld3.draw_figure(figId, spec);
}
