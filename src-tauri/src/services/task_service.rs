use crate::db::task_repo;
use crate::models::task::Task;
use rusqlite::Connection;

/// 根据视图类型过滤任务
pub fn get_filtered_tasks(
    conn: &Connection,
    view_type: &str,
    view_id: &str,
    search: &str,
) -> Result<Vec<Task>, String> {
    let all_tasks = task_repo::get_all(conn)?;

    let mut result = all_tasks;

    // 按搜索关键词过滤
    if !search.is_empty() {
        let query = search.to_lowercase();
        result = result
            .into_iter()
            .filter(|t| {
                t.title.to_lowercase().contains(&query)
                    || t.description.to_lowercase().contains(&query)
            })
            .collect();
    }

    // 按视图类型过滤
    match view_type {
        "smart" => {
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            match view_id {
                "today" => {
                    result = result
                        .into_iter()
                        .filter(|t| {
                            t.due_date
                                .as_ref()
                                .map(|d| d.starts_with(&today))
                                .unwrap_or(false)
                        })
                        .collect();
                }
                "week" => {
                    let week_later = chrono::Local::now() + chrono::Duration::days(7);
                    let week_str = week_later.format("%Y-%m-%d").to_string();
                    result = result
                        .into_iter()
                        .filter(|t| {
                            t.due_date
                                .as_ref()
                                .map(|d| {
                                    let date_part = if d.contains('T') {
                                        &d[..10]
                                    } else {
                                        d.as_str()
                                    };
                                    date_part <= week_str.as_str()
                                })
                                .unwrap_or(false)
                        })
                        .collect();
                }
                "all" => {}
                "completed" => {
                    result = result
                        .into_iter()
                        .filter(|t| t.status == crate::models::task::TaskStatus::Completed)
                        .collect();
                }
                "favorite" => {
                    result = result.into_iter().filter(|t| t.favorite).collect();
                }
                _ => {}
            }
        }
        "project" => {
            result = result
                .into_iter()
                .filter(|t| t.project_id.as_deref() == Some(view_id))
                .collect();
        }
        "tag" => {
            result = result
                .into_iter()
                .filter(|t| t.tags.contains(&view_id.to_string()))
                .collect();
        }
        _ => {}
    }

    Ok(result)
}
