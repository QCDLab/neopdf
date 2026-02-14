// Main application logic for the NeoPDF GUI frontend.

// ---------------------------------------------------------------------------
// DOM references
// ---------------------------------------------------------------------------
const $setNameInput = document.getElementById("set-name-input");
const $setList = document.getElementById("set-list");
const $btnAddSet = document.getElementById("btn-add-set");
const $btnRemoveSet = document.getElementById("btn-remove-set");
const $btnClearSets = document.getElementById("btn-clear-sets");
const $xAxisVar = document.getElementById("x-axis-var");
const $pidSelect = document.getElementById("pid-select");
const $fixedParams = document.getElementById("fixed-params-container");
const $rangeMin = document.getElementById("range-min");
const $rangeMax = document.getElementById("range-max");
const $nPoints = document.getElementById("n-points");
const $xLog = document.getElementById("x-log");
const $yLog = document.getElementById("y-log");
const $ratioOptions = document.getElementById("ratio-options");
const $ratioRef = document.getElementById("ratio-ref");
const $btnPlot = document.getElementById("btn-plot");
const $btnExport = document.getElementById("btn-export");
const $statusBar = document.getElementById("status-bar");
const $metadataContent = document.getElementById("metadata-content");
const $dataPathInput = document.getElementById("data-path-input");
const $btnBrowseDataPath = document.getElementById("btn-browse-data-path");
const $btnSaveDataPath = document.getElementById("btn-save-data-path");

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------
let activeParams = []; // Current ParamRangeInfo[]
let lastPlotRequest = null; // Saved for export

// Parton names for display
const PID_LABELS = {
  21: "g (21)",
  1: "d (1)",
  2: "u (2)",
  3: "s (3)",
  4: "c (4)",
  5: "b (5)",
  6: "t (6)",
  "-1": "dbar (-1)",
  "-2": "ubar (-2)",
  "-3": "sbar (-3)",
  "-4": "cbar (-4)",
  "-5": "bbar (-5)",
  "-6": "tbar (-6)",
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
function status(msg, cls = "") {
  $statusBar.textContent = msg;
  $statusBar.className = "status-bar " + cls;
}

function getSelectedSetNames() {
  return Array.from($setList.selectedOptions).map((o) => o.value);
}

function getAllSetNames() {
  return Array.from($setList.options).map((o) => o.value);
}

function getPlotType() {
  return document.querySelector('input[name="plot-type"]:checked').value;
}

// ---------------------------------------------------------------------------
// Set Management
// ---------------------------------------------------------------------------
async function addSet() {
  const name = $setNameInput.value.trim();
  if (!name) return;

  // Skip if already in list
  if (Array.from($setList.options).some((o) => o.value === name)) {
    status("Set already loaded", "error");
    return;
  }

  status("Loading " + name + "...");
  $btnAddSet.disabled = true;

  try {
    await api.loadPdfSet(name);
    const opt = document.createElement("option");
    opt.value = name;
    opt.textContent = name;
    $setList.appendChild(opt);
    opt.selected = true;
    $setNameInput.value = "";
    status("Loaded " + name, "success");
    await onSelectionChanged();
  } catch (e) {
    status("Error: " + e, "error");
  } finally {
    $btnAddSet.disabled = false;
  }
}

async function removeSelectedSets() {
  const names = getSelectedSetNames();
  for (const name of names) {
    try {
      await api.removePdfSet(name);
    } catch (_) {
      // best-effort
    }
    for (const opt of Array.from($setList.options)) {
      if (opt.value === name) opt.remove();
    }
  }
  await onSelectionChanged();
}

async function clearAllSets() {
  const names = getAllSetNames();
  for (const name of names) {
    try {
      await api.removePdfSet(name);
    } catch (_) {
      // best-effort
    }
  }
  $setList.innerHTML = "";
  await onSelectionChanged();
}

// ---------------------------------------------------------------------------
// Parameter Detection
// ---------------------------------------------------------------------------
async function onSelectionChanged() {
  const selected = getSelectedSetNames();
  updateRatioRefDropdown();

  if (selected.length === 0) {
    activeParams = [];
    $xAxisVar.innerHTML = "";
    $pidSelect.innerHTML = "";
    $fixedParams.innerHTML = "";
    $metadataContent.innerHTML = '<p class="placeholder">Select a set to view its metadata.</p>';
    return;
  }

  try {
    // Detect parameters and PIDs in parallel
    const [paramResult, pids] = await Promise.all([
      api.detectActiveParameters(selected),
      api.getAvailablePids(selected),
    ]);

    activeParams = paramResult.params;

    // Populate x-axis variable dropdown
    $xAxisVar.innerHTML = "";
    for (const p of activeParams) {
      if (p.active) {
        const opt = document.createElement("option");
        opt.value = p.name;
        opt.textContent = p.name;
        $xAxisVar.appendChild(opt);
      }
    }

    // Populate PID dropdown
    $pidSelect.innerHTML = "";
    for (const pid of pids) {
      const opt = document.createElement("option");
      opt.value = pid;
      opt.textContent = PID_LABELS[pid] || String(pid);
      $pidSelect.appendChild(opt);
    }

    rebuildFixedParams();

    // Show metadata for the first selected set
    await showMetadata(selected[0]);
  } catch (e) {
    status("Error detecting parameters: " + e, "error");
  }
}

function rebuildFixedParams() {
  $fixedParams.innerHTML = "";
  const xVar = $xAxisVar.value;

  for (const p of activeParams) {
    if (!p.active || p.name === xVar) continue;

    const row = document.createElement("div");
    row.className = "fixed-param-row";

    const label = document.createElement("label");
    label.textContent = "Fixed " + p.name + ":";
    row.appendChild(label);

    const input = document.createElement("input");
    input.type = "text";
    input.dataset.param = p.name;
    input.value = p.default_value;
    row.appendChild(input);

    $fixedParams.appendChild(row);
  }
}

// ---------------------------------------------------------------------------
// Metadata
// ---------------------------------------------------------------------------
async function showMetadata(name) {
  try {
    const m = await api.getSetMetadata(name);
    $metadataContent.innerHTML = `<table>
      <tr><td>Set Name</td><td>${m.set_name}</td></tr>
      <tr><td>Description</td><td>${m.set_desc}</td></tr>
      <tr><td>Members</td><td>${m.num_members}</td></tr>
      <tr><td>Error Type</td><td>${m.error_type}</td></tr>
      <tr><td>Flavor Scheme</td><td>${m.flavor_scheme}</td></tr>
      <tr><td>QCD Order</td><td>${m.order_qcd}</td></tr>
      <tr><td>x range</td><td>[${m.x_min.toExponential(2)}, ${m.x_max}]</td></tr>
      <tr><td>Q range</td><td>[${m.q_min}, ${m.q_max}]</td></tr>
      <tr><td>Flavors</td><td>${m.flavors.join(", ")}</td></tr>
      <tr><td>M_Z</td><td>${m.m_z} GeV</td></tr>
      <tr><td>M_W</td><td>${m.m_w} GeV</td></tr>
      <tr><td>M_charm</td><td>${m.m_charm} GeV</td></tr>
      <tr><td>M_bottom</td><td>${m.m_bottom} GeV</td></tr>
      <tr><td>M_top</td><td>${m.m_top} GeV</td></tr>
      <tr><td>Format</td><td>${m.format}</td></tr>
      <tr><td>Interpolator</td><td>${m.interpolator_type}</td></tr>
      <tr><td>Code Version</td><td>${m.code_version}</td></tr>
    </table>`;
  } catch (e) {
    $metadataContent.innerHTML = '<p class="placeholder">Error: ' + e + "</p>";
  }
}

// ---------------------------------------------------------------------------
// Ratio Mode
// ---------------------------------------------------------------------------
function updateRatioRefDropdown() {
  $ratioRef.innerHTML = "";
  for (const opt of $setList.options) {
    const o = document.createElement("option");
    o.value = opt.value;
    o.textContent = opt.value;
    $ratioRef.appendChild(o);
  }
}

document.querySelectorAll('input[name="plot-type"]').forEach((r) =>
  r.addEventListener("change", () => {
    $ratioOptions.classList.toggle("hidden", getPlotType() !== "ratio");
  })
);

// ---------------------------------------------------------------------------
// Plotting
// ---------------------------------------------------------------------------
function buildPlotRequest() {
  const setNames = getSelectedSetNames();
  if (setNames.length === 0) throw new Error("No PDF sets selected");

  const xAxisVar = $xAxisVar.value;
  if (!xAxisVar) throw new Error("No x-axis variable selected");

  const pid = parseInt($pidSelect.value, 10);
  const rangeMin = parseFloat($rangeMin.value);
  const rangeMax = parseFloat($rangeMax.value);
  const nPoints = parseInt($nPoints.value, 10);

  if (isNaN(rangeMin) || isNaN(rangeMax) || isNaN(nPoints) || nPoints < 2) {
    throw new Error("Invalid range or point count");
  }

  const fixedValues = {};
  for (const input of $fixedParams.querySelectorAll("input[data-param]")) {
    fixedValues[input.dataset.param] = parseFloat(input.value);
  }

  const plotType = getPlotType();
  const ratioReference = plotType === "ratio" ? $ratioRef.value || null : null;

  return {
    set_names: setNames,
    x_axis_var: xAxisVar,
    pid,
    fixed_values: fixedValues,
    range_min: rangeMin,
    range_max: rangeMax,
    n_points: nPoints,
    x_log: $xLog.checked,
    y_log: $yLog.checked,
    plot_type: plotType,
    ratio_reference: ratioReference,
  };
}

async function doPlot() {
  let request;
  try {
    request = buildPlotRequest();
  } catch (e) {
    status(e.message, "error");
    return;
  }

  status("Generating plot...");
  $btnPlot.disabled = true;

  try {
    const specJson = await api.generatePlot(request);
    embedPlot(specJson);
    lastPlotRequest = request;
    status("Plot ready", "success");
  } catch (e) {
    status("Plot error: " + e, "error");
  } finally {
    $btnPlot.disabled = false;
  }
}

async function doExport() {
  if (!lastPlotRequest) {
    status("Generate a plot first", "error");
    return;
  }

  // Use Tauri's save dialog (requires tauri-plugin-dialog and dialog:allow-save capability)
  try {
    const dialog = window.__TAURI__?.dialog;
    if (!dialog?.save) {
      status("Export: dialog plugin not available", "error");
      return;
    }
    const path = await dialog.save({
      filters: [
        { name: "PNG Image", extensions: ["png"] },
        { name: "PDF Document", extensions: ["pdf"] },
        { name: "SVG Image", extensions: ["svg"] },
      ],
    });
    if (!path) return;

    status("Exporting...");
    await api.exportPlotPng(lastPlotRequest, path);
    status("Exported to " + path, "success");
  } catch (e) {
    status("Export error: " + e, "error");
  }
}

// ---------------------------------------------------------------------------
// Settings: Grid data path
// ---------------------------------------------------------------------------
async function loadDataPathSetting() {
  try {
    const path = await api.getDataPathSetting();
    $dataPathInput.value = path || "";
  } catch (e) {
    console.warn("Failed to load data path setting:", e);
  }
}

async function browseDataPath() {
  const dialog = window.__TAURI__?.dialog;
  if (!dialog?.open) {
    status("Browse not available", "error");
    return;
  }
  try {
    const selected = await dialog.open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      $dataPathInput.value = Array.isArray(selected) ? selected[0] : selected;
    }
  } catch (e) {
    status("Browse error: " + e, "error");
  }
}

async function saveDataPath() {
  const path = $dataPathInput.value.trim() || null;
  try {
    await api.setDataPathSetting(path);
    status(path ? "Grid path saved. Reload PDF sets to use the new path." : "Grid path cleared.", "success");
  } catch (e) {
    status("Failed to save: " + e, "error");
  }
}

// ---------------------------------------------------------------------------
// Event Wiring
// ---------------------------------------------------------------------------
loadDataPathSetting();

$btnAddSet.addEventListener("click", addSet);
$setNameInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") addSet();
});
$btnRemoveSet.addEventListener("click", removeSelectedSets);
$btnClearSets.addEventListener("click", clearAllSets);
$setList.addEventListener("change", onSelectionChanged);
$xAxisVar.addEventListener("change", rebuildFixedParams);
$btnPlot.addEventListener("click", doPlot);
$btnExport.addEventListener("click", doExport);
$btnBrowseDataPath.addEventListener("click", browseDataPath);
$btnSaveDataPath.addEventListener("click", saveDataPath);
