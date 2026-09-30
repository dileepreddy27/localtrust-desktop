#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use localtrust_core::{Action, Broker, Event, Proposal};
use std::sync::Mutex;
use tauri::Manager;
#[tauri::command]
fn propose(action: Action, state: tauri::State<Mutex<Broker>>) -> Result<Proposal, String> {
    state.lock().map_err(|e| e.to_string())?.propose(action)
}
#[tauri::command]
fn decide(id: u64, approve: bool, state: tauri::State<Mutex<Broker>>) -> Result<String, String> {
    state.lock().map_err(|e| e.to_string())?.decide(id, approve)
}
#[tauri::command]
fn events(state: tauri::State<Mutex<Broker>>) -> Result<Vec<Event>, String> {
    Ok(state.lock().map_err(|e| e.to_string())?.events().to_vec())
}
#[tauri::command]
async fn plan(prompt: String, model: String) -> Result<Action, String> {
    tauri::async_runtime::spawn_blocking(move || localtrust_core::plan_local(&prompt, &model)).await.map_err(|e| e.to_string())?
}
fn main() {
    tauri::Builder::default().setup(|app| {
        let root = app.path().app_local_data_dir()?.join("workspace");
        app.manage(Mutex::new(Broker::new(root).map_err(std::io::Error::other)?));
        Ok(())
    }).invoke_handler(tauri::generate_handler![propose, decide, events, plan])
    .run(tauri::generate_context!()).expect("Unable to start LocalTrust Desktop");
}
