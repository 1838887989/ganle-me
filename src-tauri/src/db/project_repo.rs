use crate::models::project::Project;
use rusqlite::{params, Connection};

/// 获取所有项目
pub fn get_all(conn: &Connection) -> Result<Vec<Project>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name, color, icon, archived, created_at FROM projects ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                archived: row.get::<_, i32>(4)? != 0,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(projects)
}

/// 创建项目
pub fn create(conn: &Connection, id: &str, name: &str, color: &str, icon: &str) -> Result<Project, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    conn.execute(
        "INSERT INTO projects (id, name, color, icon, archived, created_at) VALUES (?1, ?2, ?3, ?4, 0, ?5)",
        params![id, name, color, icon, &now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Project {
        id: id.to_string(),
        name: name.to_string(),
        color: color.to_string(),
        icon: icon.to_string(),
        archived: false,
        created_at: now,
    })
}

/// 更新项目
pub fn update(
    conn: &Connection,
    id: &str,
    name: Option<&str>,
    color: Option<&str>,
    icon: Option<&str>,
    archived: Option<bool>,
) -> Result<Project, String> {
    if let Some(v) = name {
        conn.execute("UPDATE projects SET name = ?1 WHERE id = ?2", params![v, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = color {
        conn.execute("UPDATE projects SET color = ?1 WHERE id = ?2", params![v, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = icon {
        conn.execute("UPDATE projects SET icon = ?1 WHERE id = ?2", params![v, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = archived {
        conn.execute("UPDATE projects SET archived = ?1 WHERE id = ?2", params![v as i32, id])
            .map_err(|e| e.to_string())?;
    }

    get_by_id(conn, id)
}

/// 根据ID获取项目
pub fn get_by_id(conn: &Connection, id: &str) -> Result<Project, String> {
    conn.query_row(
        "SELECT id, name, color, icon, archived, created_at FROM projects WHERE id = ?1",
        params![id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                archived: row.get::<_, i32>(4)? != 0,
                created_at: row.get(5)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// 删除项目
pub fn delete(conn: &Connection, id: &str) -> Result<(), String> {
    // 将关联任务的 project_id 设为 NULL
    conn.execute("UPDATE tasks SET project_id = NULL WHERE project_id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取项目下未完成任务数
pub fn get_task_count(conn: &Connection, id: &str) -> Result<usize, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE project_id = ?1 AND status != 'completed'",
        params![id],
        |row| row.get(0),
    )
    .map_err(|e| e.to_string())
}
