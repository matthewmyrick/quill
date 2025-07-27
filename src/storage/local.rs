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
}