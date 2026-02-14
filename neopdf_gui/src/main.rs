//! `NeoPDF` GUI — a Tauri v2 desktop application for interactive PDF plotting.

// Prevents a console window from appearing on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod plotting;
mod settings;
mod state;
mod types;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .setup(|app| {
            settings::apply_data_path_to_env(&app.handle())
                .map_err(|e| {
                    eprintln!("Failed to apply data path from config: {}", e);
                })
                .ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_pdf_set,
            commands::remove_pdf_set,
            commands::get_loaded_sets,
            commands::detect_active_parameters,
            commands::get_set_metadata,
            commands::generate_plot,
            commands::export_plot_png,
            commands::get_available_pids,
            commands::compute_alphas,
            commands::get_data_path_setting,
            commands::set_data_path_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running NeoPDF GUI");
}
