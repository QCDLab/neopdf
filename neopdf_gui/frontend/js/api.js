// Thin wrapper around Tauri's invoke API.
//
// Each function corresponds to a #[tauri::command] in commands.rs.
// Uses `window.__TAURI__.core.invoke()` provided by Tauri's withGlobalTauri.

const { invoke } = window.__TAURI__.core;

const api = {
  async loadPdfSet(name) {
    return invoke("load_pdf_set", { name });
  },

  async removePdfSet(name) {
    return invoke("remove_pdf_set", { name });
  },

  async getLoadedSets() {
    return invoke("get_loaded_sets");
  },

  async detectActiveParameters(setNames) {
    return invoke("detect_active_parameters", { setNames });
  },

  async getSetMetadata(name) {
    return invoke("get_set_metadata", { name });
  },

  async getAvailablePids(setNames) {
    return invoke("get_available_pids", { setNames });
  },

  async computeAlphas(setName, q2Values) {
    return invoke("compute_alphas", { setName, q2Values });
  },

  async generatePlot(request) {
    return invoke("generate_plot", { request });
  },

  async exportPlotPng(request, outputPath) {
    return invoke("export_plot_png", { request, outputPath });
  },
};
