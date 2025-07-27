use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod local;
pub mod mongodb;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub text: String,
    #[serde(default)]
    pub status: TaskStatus,
    pub created_at: String,
}

impl Task {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            status: TaskStatus::NotStarted,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[allow(dead_code)]
    pub fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }
}

#[async_trait]
pub trait TaskStorage: Send + Sync {
    async fn get_tasks(&self, context_key: &str) -> Result<Vec<Task>>;
    async fn add_task(&mut self, context_key: &str, text: String) -> Result<usize>;
    async fn toggle_task(&mut self, context_key: &str, id: usize) -> Result<bool>;
    async fn set_task_status(&mut self, context_key: &str, id: usize, status: TaskStatus) -> Result<bool>;
    async fn remove_task(&mut self, context_key: &str, id: usize) -> Result<bool>;
    async fn edit_task(&mut self, context_key: &str, id: usize, new_text: String) -> Result<bool>;
    async fn undo_delete(&mut self, context_key: &str) -> Result<Option<Task>>;
}