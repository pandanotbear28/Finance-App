#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn data_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .document_dir()
        .map_err(|e| e.to_string())?
        .join("Ledger");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("ledger-data.json"))
}

#[tauri::command]
fn save_data(app: tauri::AppHandle, json: String) -> Result<(), String> {
    let path = data_path(&app)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_data(app: tauri::AppHandle) -> Option<String> {
    let path = data_path(&app).ok()?;
    fs::read_to_string(path).ok()
}

#[tauri::command]
fn data_location(app: tauri::AppHandle) -> Result<String, String> {
    Ok(data_path(&app)?.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_data, load_data, data_location])
        .run(tauri::generate_context!())
        .expect("error while running Ledger");
}
