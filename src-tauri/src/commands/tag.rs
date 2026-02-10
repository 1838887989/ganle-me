use crate::db::tag_repo;
use crate::models::tag::Tag;
use crate::AppState;
use tauri::State;
use uuid::Uuid;

/// 获取所有标签
#[tauri::command]
pub fn get_all_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    tag_repo::get_all(&db.conn)
}

/// 创建标签
#[tauri::command]
pub fn create_tag(
    state: State<'_, AppState>,
    name: String,
    color: String,
) -> Result<Tag, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let id = format!("tag-{}", Uuid::new_v4());
    tag_repo::create(&db.conn, &id, &name, &color)
}

/// 更新标签
#[tauri::command]
pub fn update_tag(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    color: Option<String>,
) -> Result<Tag, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    tag_repo::update(&db.conn, &id, name.as_deref(), color.as_deref())
}

/// 删除标签
#[tauri::command]
pub fn delete_tag(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    tag_repo::delete(&db.conn, &id)
}

/// 获取标签下未完成任务数
#[tauri::command]
pub fn get_tag_task_count(state: State<'_, AppState>, id: String) -> Result<usize, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    tag_repo::get_task_count(&db.conn, &id)
}
