use serde::{Deserialize, Serialize};

/// 任务优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        }
    }
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Completed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "in_progress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            _ => TaskStatus::Todo,
        }
    }
}

/// 重复类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RepeatType {
    Daily,
    Weekly,
    Monthly,
    Custom,
    None,
}

impl RepeatType {
    pub fn as_str(&self) -> &str {
        match self {
            RepeatType::Daily => "daily",
            RepeatType::Weekly => "weekly",
            RepeatType::Monthly => "monthly",
            RepeatType::Custom => "custom",
            RepeatType::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "daily" => RepeatType::Daily,
            "weekly" => RepeatType::Weekly,
            "monthly" => RepeatType::Monthly,
            "custom" => RepeatType::Custom,
            _ => RepeatType::None,
        }
    }
}

/// 子任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

/// 任务
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: Priority,
    pub project_id: Option<String>,
    pub tags: Vec<String>,
    pub subtasks: Vec<SubTask>,
    pub due_date: Option<String>,
    pub reminder: Option<String>,
    pub repeat: RepeatType,
    pub favorite: bool,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// 任务统计
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCounts {
    pub today_count: usize,
    pub week_count: usize,
    pub all_count: usize,
    pub completed_count: usize,
    pub favorite_count: usize,
}
