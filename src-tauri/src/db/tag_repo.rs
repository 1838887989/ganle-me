use crate::models::tag::Tag;
use rusqlite::{params, Connection};

/// 获取所有标签
pub fn get_all(conn: &Connection) -> Result<Vec<Tag>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name, color FROM tags")
        .map_err(|e| e.to_string())?;

    let tags = stmt
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(tags)
}

/// 创建标签
pub fn create(conn: &Connection, id: &str, name: &str, color: &str) -> Result<Tag, String> {
    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
        params![id, name, color],
    )
    .map_err(|e| e.to_string())?;

    Ok(Tag {
        id: id.to_string(),
        name: name.to_string(),
        color: color.to_string(),
    })
}

/// 更新标签
pub fn update(
    conn: &Connection,
    id: &str,
    name: Option<&str>,
    color: Option<&str>,
) -> Result<Tag, String> {
    if let Some(v) = name {
        conn.execute("UPDATE tags SET name = ?1 WHERE id = ?2", params![v, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(v) = color {
        conn.execute("UPDATE tags SET color = ?1 WHERE id = ?2", params![v, id])
            .map_err(|e| e.to_string())?;
    }
    get_by_id(conn, id)
}

/// 根据ID获取标签
pub fn get_by_id(conn: &Connection, id: &str) -> Result<Tag, String> {
    conn.query_row(
        "SELECT id, name, color FROM tags WHERE id = ?1",
        params![id],
        |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// 删除标签（同时清理关联）
pub fn delete(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM task_tags WHERE tag_id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tags WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取标签下未完成任务数
pub fn get_task_count(conn: &Connection, id: &str) -> Result<usize, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM task_tags tt
         JOIN tasks t ON tt.task_id = t.id
         WHERE tt.tag_id = ?1 AND t.status != 'completed'",
        params![id],
        |row| row.get(0),
    )
    .map_err(|e| e.to_string())
}
