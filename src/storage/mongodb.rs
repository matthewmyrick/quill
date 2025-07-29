use super::{Task, TaskStatus, TaskStorage};
use anyhow::Result;
use async_trait::async_trait;
use bson::doc;
use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub task_id: i64,
    pub context_key: String,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
}

impl From<(&str, &Task)> for TaskDocument {
    fn from((context_key, task): (&str, &Task)) -> Self {
        Self {
            id: None,
            task_id: task.id as i64,
            context_key: context_key.to_string(),
            text: task.text.clone(),
            status: task.status.clone(),
            created_at: task.created_at.clone(),
        }
    }
}

impl From<TaskDocument> for Task {
    fn from(doc: TaskDocument) -> Self {
        Self {
            id: doc.task_id as usize,
            text: doc.text,
            status: doc.status,
            created_at: doc.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CounterDocument {
    #[serde(rename = "_id")]
    pub id: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeletedTaskDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub context_key: String,
    pub task_id: i64,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub deleted_at: String,
}

impl From<(&str, &Task)> for DeletedTaskDocument {
    fn from((context_key, task): (&str, &Task)) -> Self {
        Self {
            id: None,
            context_key: context_key.to_string(),
            task_id: task.id as i64,
            text: task.text.clone(),
            status: task.status.clone(),
            created_at: task.created_at.clone(),
            deleted_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl From<DeletedTaskDocument> for Task {
    fn from(doc: DeletedTaskDocument) -> Self {
        Self {
            id: doc.task_id as usize,
            text: doc.text,
            status: doc.status,
            created_at: doc.created_at,
        }
    }
}

pub struct MongoTaskStorage {
    collection: Collection<TaskDocument>,
    counter_collection: Collection<CounterDocument>,
    deleted_collection: Collection<DeletedTaskDocument>,
    _db: Database,
    _client: Client,
}

impl MongoTaskStorage {
    pub async fn new(connection_string: &str, database: &str, collection: &str) -> Result<Self> {
        // Add connection timeout of 10 seconds
        let connect_future = async {
            let client = Client::with_uri_str(connection_string).await?;
            
            // Test the connection by running a simple command
            let db = client.database(database);
            db.run_command(doc! { "ping": 1 }).await?;
            
            let task_collection = db.collection::<TaskDocument>(collection);
            let counter_collection = db.collection::<CounterDocument>("counters");
            let deleted_collection = db.collection::<DeletedTaskDocument>("deleted_tasks");

            Ok::<Self, anyhow::Error>(Self {
                collection: task_collection,
                counter_collection,
                deleted_collection,
                _db: db,
                _client: client,
            })
        };
        
        timeout(Duration::from_secs(10), connect_future)
            .await
            .map_err(|_| anyhow::anyhow!("MongoDB connection timeout after 10 seconds"))?
    }

    async fn get_next_counter_value(&self) -> Result<i64> {
        let filter = doc! { "_id": "task_id" };
        let update = doc! { "$inc": { "value": 1 } };
        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .upsert(true)
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        let result = self.counter_collection
            .find_one_and_update(filter, update)
            .with_options(options)
            .await?;

        match result {
            Some(counter) => Ok(counter.value),
            None => {
                // Initialize counter if it doesn't exist
                let counter = CounterDocument {
                    id: "task_id".to_string(),
                    value: 1,
                };
                self.counter_collection.insert_one(&counter).await?;
                Ok(1)
            }
        }
    }
}

#[async_trait]
impl TaskStorage for MongoTaskStorage {
    async fn get_tasks(&self, context_key: &str) -> Result<Vec<Task>> {
        let filter = doc! { "context_key": context_key };
        let mut cursor = self.collection.find(filter).await?;
        let mut tasks = Vec::new();

        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;
            tasks.push(Task::from(doc));
        }

        // Sort by task_id to maintain order
        tasks.sort_by_key(|t| t.id);
        Ok(tasks)
    }

    async fn add_task(&mut self, context_key: &str, text: String) -> Result<usize> {
        let task_id = self.get_next_counter_value().await?;
        let task = Task::new(task_id as usize, text);
        let doc = TaskDocument::from((context_key, &task));
        
        self.collection.insert_one(&doc).await?;
        Ok(task_id as usize)
    }

    async fn toggle_task(&mut self, context_key: &str, id: usize) -> Result<bool> {
        let filter = doc! { "context_key": context_key, "task_id": id as i64 };
        
        // First, get the current task to determine next status
        if let Some(doc) = self.collection.find_one(filter.clone()).await? {
            let current_status = doc.status;
            let new_status = match current_status {
                TaskStatus::NotStarted => TaskStatus::InProgress,
                TaskStatus::InProgress => TaskStatus::Completed,
                TaskStatus::Completed => TaskStatus::NotStarted,
            };

            let update = doc! { "$set": { "status": bson::to_bson(&new_status)? } };
            let result = self.collection.update_one(filter, update).await?;
            Ok(result.modified_count > 0)
        } else {
            Ok(false)
        }
    }

    async fn set_task_status(&mut self, context_key: &str, id: usize, status: TaskStatus) -> Result<bool> {
        let filter = doc! { "context_key": context_key, "task_id": id as i64 };
        let update = doc! { "$set": { "status": bson::to_bson(&status)? } };
        
        let result = self.collection.update_one(filter, update).await?;
        Ok(result.modified_count > 0)
    }

    async fn remove_task(&mut self, context_key: &str, id: usize) -> Result<bool> {
        let filter = doc! { "context_key": context_key, "task_id": id as i64 };
        
        // First, get the task before deleting it
        if let Some(task_doc) = self.collection.find_one(filter.clone()).await? {
            let task = Task::from(task_doc);
            
            // Store the deleted task
            let deleted_doc = DeletedTaskDocument::from((context_key, &task));
            self.deleted_collection.insert_one(&deleted_doc).await?;
            
            // Clean up old deleted tasks (keep only last 3 per context)
            let cleanup_filter = doc! { "context_key": context_key };
            let sort = doc! { "deleted_at": -1 };
            let mut cursor = self.deleted_collection
                .find(cleanup_filter.clone())
                .sort(sort)
                .await?;
            
            let mut deleted_tasks = Vec::new();
            while cursor.advance().await? {
                let doc = cursor.deserialize_current()?;
                deleted_tasks.push(doc);
            }
            
            // If we have more than 3, delete the oldest ones
            if deleted_tasks.len() > 3 {
                for i in 3..deleted_tasks.len() {
                    if let Some(ref object_id) = deleted_tasks[i].id {
                        let delete_filter = doc! { "_id": object_id };
                        self.deleted_collection.delete_one(delete_filter).await?;
                    }
                }
            }
            
            // Now delete the original task
            let result = self.collection.delete_one(filter).await?;
            Ok(result.deleted_count > 0)
        } else {
            Ok(false)
        }
    }

    async fn edit_task(&mut self, context_key: &str, id: usize, new_text: String) -> Result<bool> {
        let filter = doc! { "context_key": context_key, "task_id": id as i64 };
        let update = doc! { "$set": { "text": new_text } };
        
        let result = self.collection.update_one(filter, update).await?;
        Ok(result.modified_count > 0)
    }

    async fn undo_delete(&mut self, context_key: &str) -> Result<Option<Task>> {
        let filter = doc! { "context_key": context_key };
        let sort = doc! { "deleted_at": -1 };
        
        // Find the most recently deleted task
        if let Some(deleted_doc) = self.deleted_collection
            .find_one(filter.clone())
            .sort(sort)
            .await? {
            
            let task = Task::from(deleted_doc.clone());
            
            // Restore the task to the main collection
            let task_doc = TaskDocument::from((context_key, &task));
            self.collection.insert_one(&task_doc).await?;
            
            // Remove the deleted task from the deleted collection
            if let Some(ref object_id) = deleted_doc.id {
                let delete_filter = doc! { "_id": object_id };
                self.deleted_collection.delete_one(delete_filter).await?;
            }
            
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    async fn move_task_up(&mut self, context_key: &str, id: usize) -> Result<bool> {
        // Get all tasks for this context, sorted by task_id
        let tasks = self.get_tasks(context_key).await?;
        
        // Find the position of the task to move
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            if pos > 0 {
                // Swap the task_ids to change order
                let current_task_id = tasks[pos].id;
                let prev_task_id = tasks[pos - 1].id;
                
                // Update both tasks with swapped IDs
                let filter1 = doc! { "context_key": context_key, "task_id": current_task_id as i64 };
                let update1 = doc! { "$set": { "task_id": prev_task_id as i64 } };
                
                let filter2 = doc! { "context_key": context_key, "task_id": prev_task_id as i64 };
                let update2 = doc! { "$set": { "task_id": current_task_id as i64 } };
                
                let result1 = self.collection.update_one(filter1, update1).await?;
                let result2 = self.collection.update_one(filter2, update2).await?;
                
                return Ok(result1.modified_count > 0 && result2.modified_count > 0);
            }
        }
        Ok(false)
    }

    async fn move_task_down(&mut self, context_key: &str, id: usize) -> Result<bool> {
        // Get all tasks for this context, sorted by task_id
        let tasks = self.get_tasks(context_key).await?;
        
        // Find the position of the task to move
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            if pos < tasks.len() - 1 {
                // Swap the task_ids to change order
                let current_task_id = tasks[pos].id;
                let next_task_id = tasks[pos + 1].id;
                
                // Update both tasks with swapped IDs
                let filter1 = doc! { "context_key": context_key, "task_id": current_task_id as i64 };
                let update1 = doc! { "$set": { "task_id": next_task_id as i64 } };
                
                let filter2 = doc! { "context_key": context_key, "task_id": next_task_id as i64 };
                let update2 = doc! { "$set": { "task_id": current_task_id as i64 } };
                
                let result1 = self.collection.update_one(filter1, update1).await?;
                let result2 = self.collection.update_one(filter2, update2).await?;
                
                return Ok(result1.modified_count > 0 && result2.modified_count > 0);
            }
        }
        Ok(false)
    }
}