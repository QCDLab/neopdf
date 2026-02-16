// Plot view: render a matplotlib PNG (base64-encoded) into the plot container.

/**
 * Render a matplotlib-generated PNG image into the plot container.
 *
 * The backend returns a base64-encoded PNG string. We simply create an <img>
 * element and set its src to a data URL using that string.
 *
 * @param {string} pngBase64 - The PNG image data as a base64 string.
 */
function embedPlot(pngBase64) {
  const container = document.getElementById("plot-container");
  container.innerHTML = "";

  const img = document.createElement("img");
  img.src = "data:image/png;base64," + pngBase64;
  img.alt = "NeoPDF plot";
  img.className = "plot-image";

  container.appendChild(img);
}
