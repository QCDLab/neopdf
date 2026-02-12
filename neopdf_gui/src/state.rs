//! Application state holding loaded PDF sets.

use std::collections::HashMap;
use std::sync::Mutex;

use neopdf::pdf::PDF;

/// A loaded PDF set with all its members.
pub struct LoadedPdfSet {
    /// All member PDFs (member 0 is the central value).
    pub members: Vec<PDF>,
}

/// Global application state managed by Tauri.
pub struct AppState {
    pub sets: Mutex<HashMap<String, LoadedPdfSet>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sets: Mutex::new(HashMap::new()),
        }
    }
}
