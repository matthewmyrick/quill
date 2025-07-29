use crate::{
    config::{AppConfig, StorageType},
    git::GitContext, 
    storage::{local::LocalTaskStorage, mongodb::MongoTaskStorage, TaskStorage, TaskStatus}, 
    ui::{InputMode, TaskUI}
};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

pub struct App {
    ui: TaskUI,
    storage: Box<dyn TaskStorage>,
    current_context: GitContext,
    last_context_check: Instant,
    config: AppConfig,
    storage_error: Option<String>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let mut config = AppConfig::load()?;
        let current_context = GitContext::from_current_dir()?;
        
        let mut storage_error = None;
        
        let mut success_message = None;
        let storage: Box<dyn TaskStorage> = match config.storage_type {
            StorageType::Local => {
                match LocalTaskStorage::new(config.expand_local_path()) {
                    Ok(storage) => {
                        success_message = Some("Successfully connected to local storage".to_string());
                        Box::new(storage)
                    },
                    Err(e) => {
                        storage_error = Some(format!("Local storage error: {}", e));
                        // Use default path as fallback
                        Box::new(LocalTaskStorage::new("~/.quill/storage/todos.json".to_string())?)
                    }
                }
            }
            StorageType::MongoDB => {
                match MongoTaskStorage::new(
                    &config.mongo_config.connection_string,
                    &config.mongo_config.database,
                    &config.mongo_config.collection,
                ).await {
                    Ok(storage) => {
                        success_message = Some("Successfully connected to MongoDB".to_string());
                        Box::new(storage)
                    },
                    Err(e) => {
                        storage_error = Some(format!("MongoDB connection failed: {}. Falling back to local storage.", e));
                        // Fallback to local storage
                        config.storage_type = StorageType::Local;
                        // Save the updated config
                        let _ = config.save();
                        Box::new(LocalTaskStorage::new(config.expand_local_path())?)
                    }
                }
            }
        };
        
        let mut app = Self {
            ui: TaskUI::new(),
            storage,
            current_context,
            last_context_check: Instant::now(),
            config,
            storage_error,
        };
        
        // Show storage error notification if any
        if let Some(error_msg) = &app.storage_error {
            app.ui.show_notification(error_msg.clone(), crate::ui::NotificationLevel::Error);
        }
        
        // Show success notification if storage connected successfully
        if let Some(success_msg) = success_message {
            app.ui.show_notification(success_msg, crate::ui::NotificationLevel::Success);
        }
        
        Ok(app)
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()
            .map_err(|e| anyhow::anyhow!("Failed to enable raw mode. Make sure you're running in a proper terminal. Error: {}", e))?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(|e| anyhow::anyhow!("Failed to setup terminal. Error: {}", e))?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)
            .map_err(|e| anyhow::anyhow!("Failed to create terminal. Error: {}", e))?;

        let result = self.run_app(&mut terminal).await;

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = result {
            println!("{err:?}");
        }

        Ok(())
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            // Check for context changes every second
            if self.last_context_check.elapsed() > Duration::from_secs(1) {
                if let Ok(new_context) = GitContext::from_current_dir() {
                    if new_context != self.current_context {
                        self.current_context = new_context;
                        self.ui.list_state.select(None);
                    }
                }
                self.last_context_check = Instant::now();
            }

            let tasks = self.storage.get_tasks(&self.current_context.context_key()).await?;
            
            terminal.draw(|f| {
                self.ui.render(f, &tasks, &self.current_context.context_key());
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match self.ui.input_mode {
                            InputMode::Normal => {
                                if self.handle_normal_input(key.code, key.modifiers).await? {
                                    break;
                                }
                            }
                            InputMode::Adding | InputMode::Editing => {
                                self.handle_input_mode(key.code).await?;
                            }
                            InputMode::ConfigHome => {
                                self.handle_config_home_mode(key.code).await?;
                            }
                            InputMode::ConfigStorageSelection => {
                                self.handle_storage_selection_mode(key.code).await?;
                            }
                            InputMode::ConfigLocal => {
                                self.handle_local_config_mode(key.code).await?;
                            }
                            InputMode::ConfigLocalField => {
                                self.handle_local_field_mode(key.code).await?;
                            }
                            InputMode::ConfigMongoDB => {
                                self.handle_mongodb_config_mode(key.code).await?;
                            }
                            InputMode::ConfigMongoDBField => {
                                self.handle_mongodb_field_mode(key.code).await?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_normal_input(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<bool> {
        let tasks = self.storage.get_tasks(&self.current_context.context_key()).await?;
        
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('a') => {
                self.ui.start_adding();
            }
            KeyCode::Char('c') => {
                self.ui.start_storage_config(&self.config);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    // Move task down with Ctrl+Down or Ctrl+j
                    if let Some(selected) = self.ui.list_state.selected() {
                        if let Some(task) = tasks.get(selected) {
                            if self.storage.move_task_down(&self.current_context.context_key(), task.id).await? {
                                // Adjust selection to follow the moved task
                                if selected < tasks.len() - 1 {
                                    self.ui.list_state.select(Some(selected + 1));
                                }
                            }
                        }
                    }
                } else {
                    self.ui.select_next(&tasks);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    // Move task up with Ctrl+Up or Ctrl+k
                    if let Some(selected) = self.ui.list_state.selected() {
                        if let Some(task) = tasks.get(selected) {
                            if self.storage.move_task_up(&self.current_context.context_key(), task.id).await? {
                                // Adjust selection to follow the moved task
                                if selected > 0 {
                                    self.ui.list_state.select(Some(selected - 1));
                                }
                            }
                        }
                    }
                } else {
                    self.ui.select_previous(&tasks);
                }
            }
            KeyCode::Char(' ') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(task) = tasks.get(selected) {
                        self.storage.toggle_task(&self.current_context.context_key(), task.id).await?;
                    }
                }
            }
            KeyCode::Char('1') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(task) = tasks.get(selected) {
                        self.storage.set_task_status(&self.current_context.context_key(), task.id, TaskStatus::NotStarted).await?;
                    }
                }
            }
            KeyCode::Char('2') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(task) = tasks.get(selected) {
                        self.storage.set_task_status(&self.current_context.context_key(), task.id, TaskStatus::InProgress).await?;
                    }
                }
            }
            KeyCode::Char('3') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(task) = tasks.get(selected) {
                        self.storage.set_task_status(&self.current_context.context_key(), task.id, TaskStatus::Completed).await?;
                    }
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(task) = tasks.get(selected) {
                        self.storage.remove_task(&self.current_context.context_key(), task.id).await?;
                        if selected > 0 && selected >= tasks.len() - 1 {
                            self.ui.list_state.select(Some(selected - 1));
                        }
                    }
                }
            }
            KeyCode::Char('e') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(task) = tasks.get(selected) {
                        // Don't allow editing completed tasks
                        if !matches!(task.status, TaskStatus::Completed) {
                            self.ui.start_editing(task);
                        }
                    }
                }
            }
            KeyCode::Char('u') => {
                match self.storage.undo_delete(&self.current_context.context_key()).await? {
                    Some(restored_task) => {
                        self.ui.show_notification(
                            format!("Restored task: {}", restored_task.text),
                            crate::ui::NotificationLevel::Success
                        );
                    }
                    None => {
                        self.ui.show_notification(
                            "No deleted tasks to undo (max 3 undos)".to_string(),
                            crate::ui::NotificationLevel::Error
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(false)
    }

    async fn handle_input_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                // Capture editing_id before finish_input clears it
                let editing_id = self.ui.editing_id;
                let text = self.ui.finish_input();
                if !text.trim().is_empty() {
                    match editing_id {
                        Some(id) => {
                            self.storage.edit_task(&self.current_context.context_key(), id, text).await?;
                        }
                        None => {
                            self.storage.add_task(&self.current_context.context_key(), text).await?;
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.ui.cancel_input();
            }
            KeyCode::Backspace => {
                self.ui.input_text.pop();
            }
            KeyCode::Char(c) => {
                self.ui.input_text.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_config_home_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.ui.config_home_prev();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.ui.config_home_next();
            }
            KeyCode::Enter => {
                match self.ui.config_field_index {
                    0 => {}, // Current storage - no action
                    1 => self.ui.enter_storage_selection(), // Configure Storage
                    2 => {
                        // Save & Exit
                        let new_config = self.ui.get_config();
                        new_config.save()?;
                        
                        // Recreate storage with new config
                        let storage_result = match new_config.storage_type {
                            StorageType::Local => {
                                LocalTaskStorage::new(new_config.expand_local_path())
                                    .map(|s| Box::new(s) as Box<dyn TaskStorage>)
                            }
                            StorageType::MongoDB => {
                                match MongoTaskStorage::new(
                                    &new_config.mongo_config.connection_string,
                                    &new_config.mongo_config.database,
                                    &new_config.mongo_config.collection,
                                ).await {
                                    Ok(storage) => Ok(Box::new(storage) as Box<dyn TaskStorage>),
                                    Err(e) => Err(e),
                                }
                            }
                        };
                        
                        match storage_result {
                            Ok(storage) => {
                                self.storage = storage;
                                self.config = new_config;
                                self.storage_error = None;
                                self.ui.show_notification("Storage configuration updated successfully".to_string(), crate::ui::NotificationLevel::Success);
                            }
                            Err(e) => {
                                let error_msg = format!("Failed to connect to new storage: {}. Keeping current configuration.", e);
                                self.ui.show_notification(error_msg, crate::ui::NotificationLevel::Error);
                            }
                        }
                        
                        self.ui.cancel_input();
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => {
                self.ui.cancel_input();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_storage_selection_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.ui.storage_selection_prev();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.ui.storage_selection_next();
            }
            KeyCode::Enter => {
                // Update the temp config with selected storage type
                match self.ui.storage_selection_index {
                    0 => {
                        self.ui.temp_config.storage_type = StorageType::Local;
                        self.ui.enter_local_config();
                    }
                    1 => {
                        self.ui.temp_config.storage_type = StorageType::MongoDB;
                        self.ui.enter_mongodb_config();
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => {
                self.ui.back_to_home();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_local_config_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                self.ui.start_field_edit();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.ui.back_to_home();
            }
            KeyCode::Esc => {
                self.ui.back_to_home();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_local_field_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                let value = self.ui.finish_input();
                self.ui.set_current_field_value(value);
                self.ui.input_mode = InputMode::ConfigLocal;
            }
            KeyCode::Esc => {
                self.ui.input_mode = InputMode::ConfigLocal;
                self.ui.input_text.clear();
            }
            KeyCode::Backspace => {
                self.ui.input_text.pop();
            }
            KeyCode::Char(c) => {
                self.ui.input_text.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_mongodb_config_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.ui.mongodb_config_prev();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.ui.mongodb_config_next();
            }
            KeyCode::Enter => {
                self.ui.start_field_edit();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.ui.back_to_home();
            }
            KeyCode::Esc => {
                self.ui.back_to_home();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_mongodb_field_mode(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                let value = self.ui.finish_input();
                self.ui.set_current_field_value(value);
                self.ui.input_mode = InputMode::ConfigMongoDB;
            }
            KeyCode::Esc => {
                self.ui.input_mode = InputMode::ConfigMongoDB;
                self.ui.input_text.clear();
            }
            KeyCode::Backspace => {
                self.ui.input_text.pop();
            }
            KeyCode::Char(c) => {
                self.ui.input_text.push(c);
            }
            _ => {}
        }
        Ok(())
    }
}
