use crate::db::task_repo;
use crate::models::task::{Task, TaskCounts, SubTask};
use crate::services::{repeat_service, reminder_service, task_service};
use crate::AppState;
use tauri::State;
use uuid::Uuid;

/// 获取所有任务
#[tauri::command]
pub fn get_all_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::get_all(&db.conn)
}

/// 获取过滤后的任务
#[tauri::command]
pub fn get_filtered_tasks(
    state: State<'_, AppState>,
    view_type: String,
    view_id: String,
    search: String,
) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_service::get_filtered_tasks(&db.conn, &view_type, &view_id, &search)
}

/// 创建任务
#[tauri::command]
pub fn create_task(
    state: State<'_, AppState>,
    title: String,
    priority: String,
    project_id: Option<String>,
    due_date: Option<String>,
    repeat_type: String,
    tag_ids: Vec<String>,
) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let id = format!("task-{}", Uuid::new_v4());
    task_repo::create(
        &db.conn,
        &id,
        &title,
        &priority,
        project_id.as_deref(),
        due_date.as_deref(),
        &repeat_type,
        &tag_ids,
    )
}

/// 更新任务
#[tauri::command]
pub fn update_task(
    state: State<'_, AppState>,
    id: String,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    project_id: Option<Option<String>>,
    due_date: Option<Option<String>>,
    reminder: Option<Option<String>>,
    repeat_type: Option<String>,
    tag_ids: Option<Vec<String>>,
) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::update(
        &db.conn,
        &id,
        title.as_deref(),
        description.as_deref(),
        status.as_deref(),
        priority.as_deref(),
        project_id.as_ref().map(|p| p.as_deref()),
        due_date.as_ref().map(|d| d.as_deref()),
        reminder.as_ref().map(|r| r.as_deref()),
        repeat_type.as_deref(),
        tag_ids.as_deref(),
    )
}

/// 切换任务完成状态（含重复任务生成）
#[tauri::command]
pub fn toggle_task_status(state: State<'_, AppState>, id: String) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let task = task_repo::toggle_status(&db.conn, &id)?;

    // 如果刚完成且是重复任务，生成下一个
    if task.status == crate::models::task::TaskStatus::Completed
        && task.repeat != crate::models::task::RepeatType::None
    {
        let _ = repeat_service::create_next_repeat_task(&db.conn, &task);
    }

    Ok(task)
}

/// 切换收藏状态
#[tauri::command]
pub fn toggle_favorite(state: State<'_, AppState>, id: String) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::toggle_favorite(&db.conn, &id)
}

/// 删除任务
#[tauri::command]
pub fn delete_task(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::delete(&db.conn, &id)
}

/// 重新排序任务
#[tauri::command]
pub fn reorder_tasks(state: State<'_, AppState>, ordered_ids: Vec<String>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::reorder(&db.conn, &ordered_ids)
}

/// 获取任务统计数据
#[tauri::command]
pub fn get_task_counts(state: State<'_, AppState>) -> Result<TaskCounts, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::get_counts(&db.conn)
}

/// 检查提醒
#[tauri::command]
pub fn check_reminders(state: State<'_, AppState>) -> Result<Option<Task>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    reminder_service::check_reminders(&db.conn)
}

// ========== 子任务命令 ==========

/// 添加子任务
#[tauri::command]
pub fn add_subtask(
    state: State<'_, AppState>,
    task_id: String,
    title: String,
) -> Result<SubTask, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let id = format!("sub-{}", Uuid::new_v4());
    task_repo::add_subtask(&db.conn, &id, &task_id, &title)
}

/// 切换子任务完成状态
#[tauri::command]
pub fn toggle_subtask(
    state: State<'_, AppState>,
    task_id: String,
    subtask_id: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::toggle_subtask(&db.conn, &task_id, &subtask_id)
}

/// 更新子任务标题
#[tauri::command]
pub fn update_subtask(
    state: State<'_, AppState>,
    task_id: String,
    subtask_id: String,
    title: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::update_subtask(&db.conn, &task_id, &subtask_id, &title)
}

/// 删除子任务
#[tauri::command]
pub fn delete_subtask(
    state: State<'_, AppState>,
    task_id: String,
    subtask_id: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    task_repo::delete_subtask(&db.conn, &task_id, &subtask_id)
}
