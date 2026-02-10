use crate::models::task::{Priority, RepeatType, SubTask, Task, TaskCounts, TaskStatus};
use rusqlite::{params, Connection};

/// 获取所有任务（含子任务和标签）
pub fn get_all(conn: &Connection) -> Result<Vec<Task>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, title, description, status, priority, project_id,
                    due_date, reminder, repeat_type, favorite, sort_order,
                    created_at, updated_at, completed_at
             FROM tasks ORDER BY sort_order ASC",
        )
        .map_err(|e| e.to_string())?;

    let tasks: Vec<Task> = stmt
        .query_map([], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                project_id: row.get(5)?,
                due_date: row.get(6)?,
                reminder: row.get(7)?,
                repeat_type: row.get(8)?,
                favorite: row.get::<_, i32>(9)?,
                sort_order: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
                completed_at: row.get(13)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .map(|row| {
            let subtasks = get_subtasks(conn, &row.id).unwrap_or_default();
            let tags = get_task_tags(conn, &row.id).unwrap_or_default();
            Task {
                id: row.id,
                title: row.title,
                description: row.description,
                status: TaskStatus::from_str(&row.status),
                priority: Priority::from_str(&row.priority),
                project_id: row.project_id,
                tags,
                subtasks,
                due_date: row.due_date,
                reminder: row.reminder,
                repeat: RepeatType::from_str(&row.repeat_type),
                favorite: row.favorite != 0,
                sort_order: row.sort_order,
                created_at: row.created_at,
                updated_at: row.updated_at,
                completed_at: row.completed_at,
            }
        })
        .collect();

    Ok(tasks)
}

/// 根据ID获取单个任务
pub fn get_by_id(conn: &Connection, id: &str) -> Result<Task, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, title, description, status, priority, project_id,
                    due_date, reminder, repeat_type, favorite, sort_order,
                    created_at, updated_at, completed_at
             FROM tasks WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let row = stmt
        .query_row(params![id], |row| {
            Ok(TaskRow {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                project_id: row.get(5)?,
                due_date: row.get(6)?,
                reminder: row.get(7)?,
                repeat_type: row.get(8)?,
                favorite: row.get::<_, i32>(9)?,
                sort_order: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
                completed_at: row.get(13)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let subtasks = get_subtasks(conn, &row.id).unwrap_or_default();
    let tags = get_task_tags(conn, &row.id).unwrap_or_default();

    Ok(row_to_task(row, subtasks, tags))
}

/// 创建任务
pub fn create(
    conn: &Connection,
    id: &str,
    title: &str,
    priority: &str,
    project_id: Option<&str>,
    due_date: Option<&str>,
    repeat_type: &str,
    tag_ids: &[String],
) -> Result<Task, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // 获取当前最大 sort_order
    let max_order: i32 = conn
        .query_row("SELECT COALESCE(MAX(sort_order), -1) FROM tasks", [], |row| row.get(0))
        .unwrap_or(-1);

    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, project_id,
         due_date, reminder, repeat_type, favorite, sort_order, created_at, updated_at, completed_at)
         VALUES (?1, ?2, '', 'todo', ?3, ?4, ?5, NULL, ?6, 0, ?7, ?8, ?8, NULL)",
        params![id, title, priority, project_id, due_date, repeat_type, max_order + 1, &now],
    )
    .map_err(|e| e.to_string())?;

    // 插入标签关联
    for tag_id in tag_ids {
        conn.execute(
            "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
            params![id, tag_id],
        )
        .map_err(|e| e.to_string())?;
    }

    get_by_id(conn, id)
}

/// 内部行结构（从数据库读取的原始数据）
struct TaskRow {
    id: String,
    title: String,
    description: String,
    status: String,
    priority: String,
    project_id: Option<String>,
    due_date: Option<String>,
    reminder: Option<String>,
    repeat_type: String,
    favorite: i32,
    sort_order: i32,
    created_at: String,
    updated_at: String,
    completed_at: Option<String>,
}

/// 将数据库行转换为 Task 结构体
fn row_to_task(row: TaskRow, subtasks: Vec<SubTask>, tags: Vec<String>) -> Task {
    Task {
        id: row.id,
        title: row.title,
        description: row.description,
        status: TaskStatus::from_str(&row.status),
        priority: Priority::from_str(&row.priority),
        project_id: row.project_id,
        tags,
        subtasks,
        due_date: row.due_date,
        reminder: row.reminder,
        repeat: RepeatType::from_str(&row.repeat_type),
        favorite: row.favorite != 0,
        sort_order: row.sort_order,
        created_at: row.created_at,
        updated_at: row.updated_at,
        completed_at: row.completed_at,
    }
}

/// 更新任务字段
pub fn update(
    conn: &Connection,
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    status: Option<&str>,
    priority: Option<&str>,
    project_id: Option<Option<&str>>,
    due_date: Option<Option<&str>>,
    reminder: Option<Option<&str>>,
    repeat_type: Option<&str>,
    tag_ids: Option<&[String]>,
) -> Result<Task, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    if let Some(v) = title {
        conn.execute("UPDATE tasks SET title = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = description {
        conn.execute("UPDATE tasks SET description = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = status {
        conn.execute("UPDATE tasks SET status = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = priority {
        conn.execute("UPDATE tasks SET priority = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = project_id {
        conn.execute("UPDATE tasks SET project_id = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = due_date {
        conn.execute("UPDATE tasks SET due_date = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = reminder {
        conn.execute("UPDATE tasks SET reminder = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = repeat_type {
        conn.execute("UPDATE tasks SET repeat_type = ?1, updated_at = ?2 WHERE id = ?3", params![v, &now, id])
            .map_err(|e| e.to_string())?;
    }
    // 更新标签关联
    if let Some(tags) = tag_ids {
        conn.execute("DELETE FROM task_tags WHERE task_id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        for tag_id in tags {
            conn.execute(
                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![id, tag_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    get_by_id(conn, id)
}

/// 切换任务完成状态
pub fn toggle_status(conn: &Connection, id: &str) -> Result<Task, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let current_status: String = conn
        .query_row("SELECT status FROM tasks WHERE id = ?1", params![id], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    if current_status == "completed" {
        conn.execute(
            "UPDATE tasks SET status = 'todo', completed_at = NULL, updated_at = ?1 WHERE id = ?2",
            params![&now, id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "UPDATE tasks SET status = 'completed', completed_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![&now, id],
        )
        .map_err(|e| e.to_string())?;
    }

    get_by_id(conn, id)
}

/// 切换收藏状态
pub fn toggle_favorite(conn: &Connection, id: &str) -> Result<Task, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    conn.execute(
        "UPDATE tasks SET favorite = CASE WHEN favorite = 0 THEN 1 ELSE 0 END, updated_at = ?1 WHERE id = ?2",
        params![&now, id],
    )
    .map_err(|e| e.to_string())?;

    get_by_id(conn, id)
}

/// 删除任务
pub fn delete(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 重新排序任务
pub fn reorder(conn: &Connection, ordered_ids: &[String]) -> Result<(), String> {
    for (i, id) in ordered_ids.iter().enumerate() {
        conn.execute(
            "UPDATE tasks SET sort_order = ?1 WHERE id = ?2",
            params![i as i32, id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 获取任务统计数据
pub fn get_counts(conn: &Connection) -> Result<TaskCounts, String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let today_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE due_date LIKE ?1 AND status != 'completed'",
            params![format!("{}%", today)],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let week_later = chrono::Local::now() + chrono::Duration::days(7);
    let week_str = week_later.format("%Y-%m-%d").to_string();

    let week_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE due_date IS NOT NULL AND SUBSTR(due_date, 1, 10) <= ?1 AND status != 'completed'",
            params![&week_str],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let all_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE status != 'completed'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let completed_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE status = 'completed'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let favorite_count: usize = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE favorite = 1 AND status != 'completed'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(TaskCounts {
        today_count,
        week_count,
        all_count,
        completed_count,
        favorite_count,
    })
}

// ========== 子任务操作 ==========

/// 获取任务的所有子任务
pub fn get_subtasks(conn: &Connection, task_id: &str) -> Result<Vec<SubTask>, String> {
    let mut stmt = conn
        .prepare("SELECT id, title, completed FROM subtasks WHERE task_id = ?1 ORDER BY sort_order ASC")
        .map_err(|e| e.to_string())?;

    let subtasks = stmt
        .query_map(params![task_id], |row| {
            Ok(SubTask {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(subtasks)
}

/// 添加子任务
pub fn add_subtask(conn: &Connection, id: &str, task_id: &str, title: &str) -> Result<SubTask, String> {
    let max_order: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM subtasks WHERE task_id = ?1",
            params![task_id],
            |row| row.get(0),
        )
        .unwrap_or(-1);

    conn.execute(
        "INSERT INTO subtasks (id, task_id, title, completed, sort_order) VALUES (?1, ?2, ?3, 0, ?4)",
        params![id, task_id, title, max_order + 1],
    )
    .map_err(|e| e.to_string())?;

    // 更新任务的 updated_at
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    conn.execute("UPDATE tasks SET updated_at = ?1 WHERE id = ?2", params![&now, task_id])
        .map_err(|e| e.to_string())?;

    Ok(SubTask {
        id: id.to_string(),
        title: title.to_string(),
        completed: false,
    })
}

/// 切换子任务完成状态
pub fn toggle_subtask(conn: &Connection, task_id: &str, subtask_id: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE subtasks SET completed = CASE WHEN completed = 0 THEN 1 ELSE 0 END WHERE id = ?1 AND task_id = ?2",
        params![subtask_id, task_id],
    )
    .map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    conn.execute("UPDATE tasks SET updated_at = ?1 WHERE id = ?2", params![&now, task_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新子任务标题
pub fn update_subtask(conn: &Connection, task_id: &str, subtask_id: &str, title: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE subtasks SET title = ?1 WHERE id = ?2 AND task_id = ?3",
        params![title, subtask_id, task_id],
    )
    .map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    conn.execute("UPDATE tasks SET updated_at = ?1 WHERE id = ?2", params![&now, task_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 删除子任务
pub fn delete_subtask(conn: &Connection, task_id: &str, subtask_id: &str) -> Result<(), String> {
    conn.execute(
        "DELETE FROM subtasks WHERE id = ?1 AND task_id = ?2",
        params![subtask_id, task_id],
    )
    .map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    conn.execute("UPDATE tasks SET updated_at = ?1 WHERE id = ?2", params![&now, task_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ========== 标签关联查询 ==========

/// 获取任务关联的标签ID列表
pub fn get_task_tags(conn: &Connection, task_id: &str) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT tag_id FROM task_tags WHERE task_id = ?1")
        .map_err(|e| e.to_string())?;

    let tags = stmt
        .query_map(params![task_id], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(tags)
}
