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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "Test task".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.text, "Test task");
        assert_eq!(task.status, TaskStatus::NotStarted);
        assert!(!task.created_at.is_empty());
    }

    #[test]
    fn test_task_status_default() {
        let status = TaskStatus::default();
        assert_eq!(status, TaskStatus::NotStarted);
    }

    #[test]
    fn test_task_is_completed() {
        let mut task = Task::new(1, "Test task".to_string());
        assert!(!task.is_completed());
        
        task.status = TaskStatus::Completed;
        assert!(task.is_completed());
    }

    #[test]
    fn test_task_serialization() {
        let task = Task::new(1, "Test task".to_string());
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();
        
        assert_eq!(task.id, deserialized.id);
        assert_eq!(task.text, deserialized.text);
        assert_eq!(task.status, deserialized.status);
    }

    #[test]
    fn test_task_status_variants() {
        let not_started = TaskStatus::NotStarted;
        let in_progress = TaskStatus::InProgress;
        let completed = TaskStatus::Completed;
        
        assert_ne!(not_started, in_progress);
        assert_ne!(in_progress, completed);
        assert_ne!(not_started, completed);
    }
}