use crate::db::project_repo;
use crate::models::project::Project;
use crate::AppState;
use tauri::State;
use uuid::Uuid;

/// 获取所有项目
#[tauri::command]
pub fn get_all_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    project_repo::get_all(&db.conn)
}

/// 创建项目
#[tauri::command]
pub fn create_project(
    state: State<'_, AppState>,
    name: String,
    color: String,
    icon: String,
) -> Result<Project, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let id = format!("proj-{}", Uuid::new_v4());
    project_repo::create(&db.conn, &id, &name, &color, &icon)
}

/// 更新项目
#[tauri::command]
pub fn update_project(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    archived: Option<bool>,
) -> Result<Project, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    project_repo::update(
        &db.conn,
        &id,
        name.as_deref(),
        color.as_deref(),
        icon.as_deref(),
        archived,
    )
}

/// 删除项目
#[tauri::command]
pub fn delete_project(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    project_repo::delete(&db.conn, &id)
}

/// 获取项目下未完成任务数
#[tauri::command]
pub fn get_project_task_count(state: State<'_, AppState>, id: String) -> Result<usize, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    project_repo::get_task_count(&db.conn, &id)
}
