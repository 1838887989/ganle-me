use crate::db::task_repo;
use crate::models::task::{Task, TaskStatus};
use rusqlite::Connection;
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

/// 已提醒的任务ID集合（避免重复提醒）
static REMINDED_IDS: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// 检查是否有需要提醒的任务
/// 返回第一个需要提醒的任务（截止前5分钟内）
pub fn check_reminders(conn: &Connection) -> Result<Option<Task>, String> {
    let tasks = task_repo::get_all(conn)?;
    let now = chrono::Local::now();
    let five_minutes = chrono::Duration::minutes(5);

    let mut reminded = REMINDED_IDS.lock().map_err(|e| e.to_string())?;

    for task in &tasks {
        if task.status == TaskStatus::Completed {
            continue;
        }
        if reminded.contains(&task.id) {
            continue;
        }

        let due_date = match &task.due_date {
            Some(d) => d,
            None => continue,
        };

        // 解析截止时间
        let due_time = parse_due_date(due_date);
        let due_time = match due_time {
            Some(t) => t,
            None => continue,
        };

        let diff = due_time - now;

        // 剩余 ≤ 5分钟且未过期
        if diff > chrono::Duration::zero() && diff <= five_minutes {
            reminded.insert(task.id.clone());
            return Ok(Some(task.clone()));
        }
    }

    Ok(None)
}

/// 解析截止日期字符串为本地时间
fn parse_due_date(due_date: &str) -> Option<chrono::DateTime<chrono::Local>> {
    if due_date.contains('T') {
        // 格式: YYYY-MM-DDTHH:mm:ss
        let parts: Vec<&str> = due_date.split('T').collect();
        if parts.len() != 2 {
            return None;
        }
        let date = chrono::NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").ok()?;
        let time = chrono::NaiveTime::parse_from_str(parts[1], "%H:%M:%S")
            .or_else(|_| chrono::NaiveTime::parse_from_str(parts[1], "%H:%M"))
            .ok()?;
        let naive = chrono::NaiveDateTime::new(date, time);
        Some(naive.and_local_timezone(chrono::Local).single()?)
    } else {
        // 格式: YYYY-MM-DD（默认23:59:59）
        let date = chrono::NaiveDate::parse_from_str(due_date, "%Y-%m-%d").ok()?;
        let time = chrono::NaiveTime::from_hms_opt(23, 59, 59)?;
        let naive = chrono::NaiveDateTime::new(date, time);
        Some(naive.and_local_timezone(chrono::Local).single()?)
    }
}
