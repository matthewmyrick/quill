use super::{Task, TaskStatus, TaskStorage};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalTaskStorage {
    pub contexts: HashMap<String, Vec<Task>>,
    pub next_id: usize,
    #[serde(default)]
    pub deleted_tasks: HashMap<String, VecDeque<Task>>,
    storage_path: PathBuf,
}

impl LocalTaskStorage {
    pub fn new(path: String) -> Result<Self> {
        let storage_path = if path.starts_with("~/") {
            let home = dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
            PathBuf::from(path.replacen("~", &home.to_string_lossy(), 1))
        } else {
            PathBuf::from(path)
        };

        let mut storage = Self {
            contexts: HashMap::new(),
            next_id: 1,
            deleted_tasks: HashMap::new(),
            storage_path,
        };

        storage.load()?;
        Ok(storage)
    }

    fn load(&mut self) -> Result<()> {
        if self.storage_path.exists() {
            let content = fs::read_to_string(&self.storage_path)?;
            let data: LocalTaskStorage = serde_json::from_str(&content)?;
            self.contexts = data.contexts;
            self.next_id = data.next_id;
            self.deleted_tasks = data.deleted_tasks;
        }
        Ok(())
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&self.storage_path, content)?;
        Ok(())
    }
}

#[async_trait]
impl TaskStorage for LocalTaskStorage {
    async fn get_tasks(&self, context_key: &str) -> Result<Vec<Task>> {
        Ok(self.contexts
            .get(context_key)
            .cloned()
            .unwrap_or_default())
    }

    async fn add_task(&mut self, context_key: &str, text: String) -> Result<usize> {
        let task = Task::new(self.next_id, text);
        let id = task.id;
        
        self.contexts
            .entry(context_key.to_string())
            .or_insert_with(Vec::new)
            .push(task);
        
        self.next_id += 1;
        self.save()?;
        Ok(id)
    }

    async fn toggle_task(&mut self, context_key: &str, id: usize) -> Result<bool> {
        if let Some(tasks) = self.contexts.get_mut(context_key) {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.status = match task.status {
                    TaskStatus::NotStarted => TaskStatus::InProgress,
                    TaskStatus::InProgress => TaskStatus::Completed,
                    TaskStatus::Completed => TaskStatus::NotStarted,
                };
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn set_task_status(&mut self, context_key: &str, id: usize, status: TaskStatus) -> Result<bool> {
        if let Some(tasks) = self.contexts.get_mut(context_key) {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.status = status;
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn remove_task(&mut self, context_key: &str, id: usize) -> Result<bool> {
        if let Some(tasks) = self.contexts.get_mut(context_key) {
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                let removed_task = tasks.remove(pos);
                
                // Store the deleted task for undo functionality (limit to 3)
                let deleted_deque = self.deleted_tasks
                    .entry(context_key.to_string())
                    .or_insert_with(VecDeque::new);
                
                deleted_deque.push_front(removed_task);
                
                // Keep only the last 3 deleted tasks
                while deleted_deque.len() > 3 {
                    deleted_deque.pop_back();
                }
                
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn edit_task(&mut self, context_key: &str, id: usize, new_text: String) -> Result<bool> {
        if let Some(tasks) = self.contexts.get_mut(context_key) {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.text = new_text;
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn undo_delete(&mut self, context_key: &str) -> Result<Option<Task>> {
        if let Some(deleted_deque) = self.deleted_tasks.get_mut(context_key) {
            if let Some(task) = deleted_deque.pop_front() {
                // Restore the task to the context
                self.contexts
                    .entry(context_key.to_string())
                    .or_insert_with(Vec::new)
                    .push(task.clone());
                
                self.save()?;
                return Ok(Some(task));
            }
        }
        Ok(None)
    }

    async fn move_task_up(&mut self, context_key: &str, id: usize) -> Result<bool> {
        if let Some(tasks) = self.contexts.get_mut(context_key) {
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                if pos > 0 {
                    tasks.swap(pos, pos - 1);
                    self.save()?;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    async fn move_task_down(&mut self, context_key: &str, id: usize) -> Result<bool> {
        if let Some(tasks) = self.contexts.get_mut(context_key) {
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                if pos < tasks.len() - 1 {
                    tasks.swap(pos, pos + 1);
                    self.save()?;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_storage() -> LocalTaskStorage {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_todos.json");
        LocalTaskStorage::new(path.to_string_lossy().to_string()).unwrap()
    }

    #[tokio::test]
    async fn test_add_and_get_tasks() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id = storage.add_task(context, "Test task".to_string()).await.unwrap();
        assert_eq!(id, 1);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].text, "Test task");
        assert_eq!(tasks[0].id, 1);
    }

    #[tokio::test]
    async fn test_toggle_task_status() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id = storage.add_task(context, "Test task".to_string()).await.unwrap();
        
        let success = storage.toggle_task(context, id).await.unwrap();
        assert!(success);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks[0].status, TaskStatus::InProgress);
        
        storage.toggle_task(context, id).await.unwrap();
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks[0].status, TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_set_task_status() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id = storage.add_task(context, "Test task".to_string()).await.unwrap();
        
        let success = storage.set_task_status(context, id, TaskStatus::Completed).await.unwrap();
        assert!(success);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks[0].status, TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_remove_and_undo_task() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id = storage.add_task(context, "Test task".to_string()).await.unwrap();
        
        let success = storage.remove_task(context, id).await.unwrap();
        assert!(success);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks.len(), 0);
        
        let restored = storage.undo_delete(context).await.unwrap();
        assert!(restored.is_some());
        assert_eq!(restored.unwrap().text, "Test task");
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_edit_task() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id = storage.add_task(context, "Original task".to_string()).await.unwrap();
        
        let success = storage.edit_task(context, id, "Edited task".to_string()).await.unwrap();
        assert!(success);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks[0].text, "Edited task");
    }

    #[tokio::test]
    async fn test_multiple_contexts() {
        let mut storage = create_test_storage();
        let context1 = "test:repo1:main";
        let context2 = "test:repo2:main";
        
        storage.add_task(context1, "Task 1".to_string()).await.unwrap();
        storage.add_task(context2, "Task 2".to_string()).await.unwrap();
        
        let tasks1 = storage.get_tasks(context1).await.unwrap();
        let tasks2 = storage.get_tasks(context2).await.unwrap();
        
        assert_eq!(tasks1.len(), 1);
        assert_eq!(tasks2.len(), 1);
        assert_eq!(tasks1[0].text, "Task 1");
        assert_eq!(tasks2[0].text, "Task 2");
    }

    #[tokio::test]
    async fn test_deleted_tasks_limit() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        for i in 1..=5 {
            let id = storage.add_task(context, format!("Task {}", i)).await.unwrap();
            storage.remove_task(context, id).await.unwrap();
        }
        
        let deleted_count = storage.deleted_tasks.get(context).map(|d| d.len()).unwrap_or(0);
        assert_eq!(deleted_count, 3); // Should be limited to 3
    }

    #[tokio::test]
    async fn test_move_task_up() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id1 = storage.add_task(context, "Task 1".to_string()).await.unwrap();
        let id2 = storage.add_task(context, "Task 2".to_string()).await.unwrap();
        let id3 = storage.add_task(context, "Task 3".to_string()).await.unwrap();
        
        // Move task 2 up (should swap with task 1)
        let success = storage.move_task_up(context, id2).await.unwrap();
        assert!(success);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks[0].text, "Task 2");
        assert_eq!(tasks[1].text, "Task 1");
        assert_eq!(tasks[2].text, "Task 3");
        
        // Try to move first task up (should fail)
        let success = storage.move_task_up(context, id2).await.unwrap();
        assert!(!success);
    }

    #[tokio::test]
    async fn test_move_task_down() {
        let mut storage = create_test_storage();
        let context = "test:repo:main";
        
        let id1 = storage.add_task(context, "Task 1".to_string()).await.unwrap();
        let id2 = storage.add_task(context, "Task 2".to_string()).await.unwrap();
        let id3 = storage.add_task(context, "Task 3".to_string()).await.unwrap();
        
        // Move task 2 down (should swap with task 3)
        let success = storage.move_task_down(context, id2).await.unwrap();
        assert!(success);
        
        let tasks = storage.get_tasks(context).await.unwrap();
        assert_eq!(tasks[0].text, "Task 1");
        assert_eq!(tasks[1].text, "Task 3");
        assert_eq!(tasks[2].text, "Task 2");
        
        // Try to move last task down (should fail)
        let success = storage.move_task_down(context, id2).await.unwrap();
        assert!(!success);
    }
}