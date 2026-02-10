use rusqlite::Connection;

/// 执行数据库迁移：建表 + 插入默认数据
pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    create_tables(conn)?;
    seed_default_data(conn)?;
    Ok(())
}

/// 创建所有表
fn create_tables(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL DEFAULT '#228B22',
            icon TEXT NOT NULL DEFAULT 'briefcase',
            archived INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL DEFAULT '#3B82F6'
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'todo',
            priority TEXT NOT NULL DEFAULT 'medium',
            project_id TEXT,
            due_date TEXT,
            reminder TEXT,
            repeat_type TEXT NOT NULL DEFAULT 'none',
            favorite INTEGER NOT NULL DEFAULT 0,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            completed_at TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
        );

        CREATE TABLE IF NOT EXISTS subtasks (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            sort_order INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS task_tags (
            task_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (task_id, tag_id),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

/// 插入默认示例数据（仅在表为空时）
fn seed_default_data(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // 检查是否已有数据
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // 插入默认项目
    conn.execute(
        "INSERT INTO projects (id, name, color, icon, archived, created_at) VALUES (?1, ?2, ?3, ?4, 0, ?5)",
        rusqlite::params!["proj-1", "工作项目", "#228B22", "briefcase", &now],
    )?;
    conn.execute(
        "INSERT INTO projects (id, name, color, icon, archived, created_at) VALUES (?1, ?2, ?3, ?4, 0, ?5)",
        rusqlite::params!["proj-2", "技术学习", "#6B7B3C", "book-open", &now],
    )?;
    conn.execute(
        "INSERT INTO projects (id, name, color, icon, archived, created_at) VALUES (?1, ?2, ?3, ?4, 0, ?5)",
        rusqlite::params!["proj-3", "个人生活", "#CA8A04", "home", &now],
    )?;

    // 插入默认标签
    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
        rusqlite::params!["tag-1", "重要", "#EF4444"],
    )?;
    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
        rusqlite::params!["tag-2", "会议", "#3B82F6"],
    )?;
    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
        rusqlite::params!["tag-3", "学习", "#8B5CF6"],
    )?;

    // 插入默认任务
    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, project_id, due_date, reminder, repeat_type, favorite, sort_order, created_at, updated_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, NULL, 'none', 1, 0, ?7, ?7, NULL)",
        rusqlite::params![
            "1",
            "完成项目提案文档",
            "撰写 Q1 产品规划提案，包含功能列表和技术方案",
            "in_progress",
            "high",
            &today,
            &now,
        ],
    )?;
    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, project_id, due_date, reminder, repeat_type, favorite, sort_order, created_at, updated_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, NULL, 'weekly', 0, 1, ?7, ?7, NULL)",
        rusqlite::params![
            "2",
            "团队周会准备",
            "准备本周进度汇报材料",
            "todo",
            "medium",
            &today,
            &now,
        ],
    )?;
    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, project_id, due_date, reminder, repeat_type, favorite, sort_order, created_at, updated_at, completed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, NULL, 'none', 0, 2, ?7, ?7, NULL)",
        rusqlite::params![
            "3",
            "阅读 Vue 3 官方文档",
            "学习 Composition API 和新特性",
            "todo",
            "low",
            "2024-01-22",
            &now,
        ],
    )?;

    // 插入默认子任务
    conn.execute(
        "INSERT INTO subtasks (id, task_id, title, completed, sort_order) VALUES (?1, ?2, ?3, 1, 0)",
        rusqlite::params!["sub-1", "1", "整理需求文档"],
    )?;
    conn.execute(
        "INSERT INTO subtasks (id, task_id, title, completed, sort_order) VALUES (?1, ?2, ?3, 0, 1)",
        rusqlite::params!["sub-2", "1", "绘制技术架构图"],
    )?;

    // 插入默认任务-标签关联
    conn.execute(
        "INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
        rusqlite::params!["1", "tag-1"],
    )?;
    conn.execute(
        "INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
        rusqlite::params!["2", "tag-2"],
    )?;
    conn.execute(
        "INSERT INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
        rusqlite::params!["3", "tag-3"],
    )?;

    // 插入默认设置
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('is_dark', 'false')",
        [],
    )?;

    Ok(())
}
