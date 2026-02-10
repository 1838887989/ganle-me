use crate::models::settings::AppSettings;
use rusqlite::{params, Connection};

/// 获取应用设置
pub fn get_settings(conn: &Connection) -> Result<AppSettings, String> {
    let is_dark: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'is_dark'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "false".to_string());

    Ok(AppSettings {
        is_dark: is_dark == "true",
    })
}

/// 更新设置
pub fn update_setting(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
