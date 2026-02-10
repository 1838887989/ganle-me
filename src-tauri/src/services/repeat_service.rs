use crate::db::task_repo;
use crate::models::task::Task;
use chrono::Datelike;
use rusqlite::Connection;
use uuid::Uuid;

/// 完成重复任务时，生成下一个任务
pub fn create_next_repeat_task(conn: &Connection, task: &Task) -> Result<Option<Task>, String> {
    if task.repeat == crate::models::task::RepeatType::None {
        return Ok(None);
    }

    let due_date = match &task.due_date {
        Some(d) => d.clone(),
        None => return Ok(None),
    };

    // 解析日期部分
    let date_str = if due_date.contains('T') {
        &due_date[..10]
    } else {
        &due_date
    };

    let parsed = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;

    // 根据重复类型计算下一个日期
    let next_date = match task.repeat {
        crate::models::task::RepeatType::Daily => parsed + chrono::Duration::days(1),
        crate::models::task::RepeatType::Weekly => parsed + chrono::Duration::days(7),
        crate::models::task::RepeatType::Monthly => {
            // 月份加1
            let month = parsed.month();
            let year = parsed.year();
            let (new_year, new_month) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
            chrono::NaiveDate::from_ymd_opt(new_year, new_month, parsed.day())
                .unwrap_or(parsed + chrono::Duration::days(30))
        }
        _ => return Ok(None),
    };

    let new_id = format!("task-{}", Uuid::new_v4());
    let next_due = next_date.format("%Y-%m-%d").to_string();

    // 创建新任务
    let new_task = task_repo::create(
        conn,
        &new_id,
        &task.title,
        task.priority.as_str(),
        task.project_id.as_deref(),
        Some(&next_due),
        task.repeat.as_str(),
        &task.tags,
    )?;

    Ok(Some(new_task))
}
