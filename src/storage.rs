use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    NotStarted,
    InProgress,
    Completed,
}

impl Default for TodoStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    #[serde(default)]
    pub status: TodoStatus,
    pub created_at: String,
}

impl Todo {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            status: TodoStatus::NotStarted,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, TodoStatus::Completed)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoStorage {
    pub contexts: HashMap<String, Vec<Todo>>,
    pub next_id: usize,
}

impl Default for TodoStorage {
    fn default() -> Self {
        Self {
            contexts: HashMap::new(),
            next_id: 1,
        }
    }
}

impl TodoStorage {
    pub fn load() -> Result<Self> {
        let path = Self::get_storage_path()?;
        
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let storage: TodoStorage = serde_json::from_str(&content)?;
            Ok(storage)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_storage_path()?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    fn get_storage_path() -> Result<PathBuf> {
        let mut path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        path.push(".quill");
        path.push("todos.json");
        Ok(path)
    }

    pub fn get_todos(&self, context_key: &str) -> Vec<Todo> {
        self.contexts
            .get(context_key)
            .cloned()
            .unwrap_or_default()
    }

    pub fn add_todo(&mut self, context_key: &str, text: String) -> Result<usize> {
        let todo = Todo::new(self.next_id, text);
        let id = todo.id;
        
        self.contexts
            .entry(context_key.to_string())
            .or_insert_with(Vec::new)
            .push(todo);
        
        self.next_id += 1;
        self.save()?;
        Ok(id)
    }

    pub fn toggle_todo(&mut self, context_key: &str, id: usize) -> Result<bool> {
        if let Some(todos) = self.contexts.get_mut(context_key) {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.status = match todo.status {
                    TodoStatus::NotStarted => TodoStatus::InProgress,
                    TodoStatus::InProgress => TodoStatus::Completed,
                    TodoStatus::Completed => TodoStatus::NotStarted,
                };
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn set_todo_status(&mut self, context_key: &str, id: usize, status: TodoStatus) -> Result<bool> {
        if let Some(todos) = self.contexts.get_mut(context_key) {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.status = status;
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn remove_todo(&mut self, context_key: &str, id: usize) -> Result<bool> {
        if let Some(todos) = self.contexts.get_mut(context_key) {
            if let Some(pos) = todos.iter().position(|t| t.id == id) {
                todos.remove(pos);
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn edit_todo(&mut self, context_key: &str, id: usize, new_text: String) -> Result<bool> {
        if let Some(todos) = self.contexts.get_mut(context_key) {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.text = new_text;
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }
}