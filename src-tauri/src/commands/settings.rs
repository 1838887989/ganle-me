use crate::db::settings_repo;
use crate::models::settings::AppSettings;
use crate::AppState;
use tauri::State;

/// 获取应用设置
#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    settings_repo::get_settings(&db.conn)
}

/// 更新设置
#[tauri::command]
pub fn update_settings(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    settings_repo::update_setting(&db.conn, &key, &value)
}
