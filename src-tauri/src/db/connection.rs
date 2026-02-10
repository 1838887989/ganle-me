use rusqlite::Connection;
use std::path::Path;

/// 数据库连接管理
pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// 创建新的数据库连接
    pub fn new(db_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;
        // 启用外键约束
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        Ok(Self { conn })
    }

    /// 执行数据库迁移
    pub fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        super::migration::run(&self.conn)?;
        Ok(())
    }
}
