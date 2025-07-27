use crate::storage::{Task, TaskStatus};
use crate::config::{AppConfig, StorageType};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

pub struct TaskUI {
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input_text: String,
    pub editing_id: Option<usize>,
    pub config_field_index: usize,
    pub temp_config: AppConfig,
    pub config_screen: ConfigScreen,
    pub storage_selection_index: usize,
    pub notification: Option<Notification>,
}

#[derive(Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub created_at: Instant,
    pub duration: Duration,
}

#[derive(Clone)]
pub enum NotificationLevel {
    Success,
    Error,
}

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Adding,
    Editing,
    ConfigHome,
    ConfigStorageSelection,
    ConfigLocal,
    ConfigLocalField,
    ConfigMongoDB,
    ConfigMongoDBField,
}

#[derive(PartialEq, Clone)]
pub enum ConfigScreen {
    Home,
    StorageSelection,
    LocalConfig,
    MongoDBConfig,
}

impl Default for TaskUI {
    fn default() -> Self {
        Self {
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            input_text: String::new(),
            editing_id: None,
            config_field_index: 0,
            temp_config: AppConfig::default(),
            config_screen: ConfigScreen::Home,
            storage_selection_index: 0,
            notification: None,
        }
    }
}

impl TaskUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_next(&mut self, tasks: &[Task]) {
        if tasks.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let next = if selected >= tasks.len() - 1 {
            0
        } else {
            selected + 1
        };
        self.list_state.select(Some(next));
    }

    pub fn select_previous(&mut self, tasks: &[Task]) {
        if tasks.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let previous = if selected == 0 {
            tasks.len() - 1
        } else {
            selected - 1
        };
        self.list_state.select(Some(previous));
    }

    pub fn start_adding(&mut self) {
        self.input_mode = InputMode::Adding;
        self.input_text.clear();
    }

    pub fn start_editing(&mut self, task: &Task) {
        self.input_mode = InputMode::Editing;
        self.input_text = task.text.clone();
        self.editing_id = Some(task.id);
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_text.clear();
        self.editing_id = None;
    }

    pub fn finish_input(&mut self) -> String {
        let text = self.input_text.clone();
        self.cancel_input();
        text
    }

    pub fn start_storage_config(&mut self, current_config: &AppConfig) {
        self.input_mode = InputMode::ConfigHome;
        self.temp_config = current_config.clone();
        self.config_screen = ConfigScreen::Home;
        self.config_field_index = 0;
        self.storage_selection_index = match current_config.storage_type {
            StorageType::Local => 0,
            StorageType::MongoDB => 1,
        };
    }

    // Navigation methods for different config screens
    pub fn config_home_next(&mut self) {
        self.config_field_index = (self.config_field_index + 1) % 3; // Current, Configure, Save
    }

    pub fn config_home_prev(&mut self) {
        self.config_field_index = if self.config_field_index == 0 { 2 } else { self.config_field_index - 1 };
    }

    pub fn storage_selection_next(&mut self) {
        self.storage_selection_index = (self.storage_selection_index + 1) % 2; // Local, MongoDB
    }

    pub fn storage_selection_prev(&mut self) {
        self.storage_selection_index = if self.storage_selection_index == 0 { 1 } else { 0 };
    }


    pub fn mongodb_config_next(&mut self) {
        self.config_field_index = (self.config_field_index + 1) % 3; // URL, Database, Collection
    }

    pub fn mongodb_config_prev(&mut self) {
        self.config_field_index = if self.config_field_index == 0 { 2 } else { self.config_field_index - 1 };
    }

    pub fn get_current_field_value(&self) -> String {
        match self.config_screen {
            ConfigScreen::LocalConfig => {
                self.temp_config.local_config.path.clone()
            }
            ConfigScreen::MongoDBConfig => {
                match self.config_field_index {
                    0 => self.temp_config.mongo_config.connection_string.clone(),
                    1 => self.temp_config.mongo_config.database.clone(),
                    2 => self.temp_config.mongo_config.collection.clone(),
                    _ => String::new(),
                }
            }
            _ => String::new(),
        }
    }

    pub fn set_current_field_value(&mut self, value: String) {
        match self.config_screen {
            ConfigScreen::LocalConfig => {
                self.temp_config.local_config.path = value;
            }
            ConfigScreen::MongoDBConfig => {
                match self.config_field_index {
                    0 => self.temp_config.mongo_config.connection_string = value,
                    1 => self.temp_config.mongo_config.database = value,
                    2 => self.temp_config.mongo_config.collection = value,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn enter_storage_selection(&mut self) {
        self.config_screen = ConfigScreen::StorageSelection;
        self.input_mode = InputMode::ConfigStorageSelection;
    }

    pub fn enter_local_config(&mut self) {
        self.config_screen = ConfigScreen::LocalConfig;
        self.input_mode = InputMode::ConfigLocal;
        self.config_field_index = 0;
    }

    pub fn enter_mongodb_config(&mut self) {
        self.config_screen = ConfigScreen::MongoDBConfig;
        self.input_mode = InputMode::ConfigMongoDB;
        self.config_field_index = 0;
    }

    pub fn start_field_edit(&mut self) {
        match self.config_screen {
            ConfigScreen::LocalConfig => {
                self.input_mode = InputMode::ConfigLocalField;
                self.input_text = self.get_current_field_value();
            }
            ConfigScreen::MongoDBConfig => {
                self.input_mode = InputMode::ConfigMongoDBField;
                self.input_text = self.get_current_field_value();
            }
            _ => {}
        }
    }

    pub fn back_to_home(&mut self) {
        self.config_screen = ConfigScreen::Home;
        self.input_mode = InputMode::ConfigHome;
        self.config_field_index = 0;
    }

    pub fn get_config(&self) -> AppConfig {
        self.temp_config.clone()
    }

    pub fn show_notification(&mut self, message: String, level: NotificationLevel) {
        self.notification = Some(Notification {
            message,
            level,
            created_at: Instant::now(),
            duration: Duration::from_secs(5),
        });
    }

    pub fn clear_expired_notification(&mut self) {
        if let Some(ref notification) = self.notification {
            if notification.created_at.elapsed() >= notification.duration {
                self.notification = None;
            }
        }
    }

    pub fn render(&mut self, f: &mut Frame, tasks: &[Task], context: &str) {
        // Clear expired notifications
        self.clear_expired_notification();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Header
        let header = Paragraph::new(format!("Quill Task - {}", context))
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Task List
        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| {
                let (symbol, style) = match task.status {
                    TaskStatus::NotStarted => ("○", Style::default().fg(Color::Yellow)),
                    TaskStatus::InProgress => ("◐", Style::default().fg(Color::Blue)),
                    TaskStatus::Completed => ("✓", Style::default().fg(Color::Green).add_modifier(Modifier::CROSSED_OUT)),
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!("{} ", symbol), style),
                    Span::styled(&task.text, style),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Tasks"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("➤ ");

        f.render_stateful_widget(list, chunks[1], &mut self.list_state);

        // Footer
        let footer_text = "Press 'a' to add, 'e' to edit (not completed), 'd' to delete, 'u' to undo delete, Space to cycle status, '1'=Not Started, '2'=In Progress, '3'=Completed, 'c' for config, 'q' to quit";
        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(footer, chunks[2]);

        // Floating input box
        match self.input_mode {
            InputMode::Adding | InputMode::Editing | InputMode::ConfigLocalField | InputMode::ConfigMongoDBField => {
                let popup_area = self.centered_rect(60, 20, f.area());
                f.render_widget(Clear, popup_area);
                
                let title = match self.input_mode {
                    InputMode::Adding => "Add New Task",
                    InputMode::Editing => "Edit Task",
                    InputMode::ConfigLocalField => "Edit Local Path",
                    InputMode::ConfigMongoDBField => "Edit MongoDB Field",
                    _ => "",
                };
                
                let input_block = Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan));
                
                let input_paragraph = Paragraph::new(self.input_text.as_str())
                    .block(input_block)
                    .wrap(Wrap { trim: false });
                
                f.render_widget(input_paragraph, popup_area);
                
                // Show cursor
                f.set_cursor_position((
                    popup_area.x + self.input_text.len() as u16 + 1,
                    popup_area.y + 1,
                ));
            }
            InputMode::ConfigHome => {
                self.render_config_home(f);
            }
            InputMode::ConfigStorageSelection => {
                self.render_storage_selection(f);
            }
            InputMode::ConfigLocal => {
                self.render_local_config(f);
            }
            InputMode::ConfigMongoDB => {
                self.render_mongodb_config(f);
            }
            _ => {}
        }

        // Render notification if present
        if let Some(ref notification) = self.notification {
            self.render_notification(f, notification);
        }
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    fn render_config_home(&self, f: &mut Frame) {
        let popup_area = self.centered_rect(70, 50, f.area());
        f.render_widget(Clear, popup_area);

        let home_block = Block::default()
            .title("Storage Configuration")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        let current_storage = match self.temp_config.storage_type {
            StorageType::Local => "Local",
            StorageType::MongoDB => "MongoDB",
        };

        let options = vec![
            format!("Current Storage: {}", current_storage),
            "Configure Storage".to_string(),
            "Save & Exit".to_string(),
        ];

        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.config_field_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(option.as_str()).style(style)
            })
            .collect();

        let home_list = List::new(items)
            .block(home_block)
            .highlight_style(Style::default().bg(Color::Blue));

        f.render_widget(home_list, popup_area);

        self.render_instructions(f, popup_area, "↑/↓: Navigate, Enter: Select, Esc: Cancel");
    }

    fn render_storage_selection(&self, f: &mut Frame) {
        let popup_area = self.centered_rect(60, 40, f.area());
        f.render_widget(Clear, popup_area);

        let selection_block = Block::default()
            .title("Select Storage Type")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        let storage_types = vec!["Local", "MongoDB"];

        let items: Vec<ListItem> = storage_types
            .iter()
            .enumerate()
            .map(|(i, storage_type)| {
                let style = if i == self.storage_selection_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(*storage_type).style(style)
            })
            .collect();

        let selection_list = List::new(items)
            .block(selection_block)
            .highlight_style(Style::default().bg(Color::Blue));

        f.render_widget(selection_list, popup_area);

        self.render_instructions(f, popup_area, "↑/↓: Navigate, Enter: Select, Esc: Back");
    }

    fn render_local_config(&self, f: &mut Frame) {
        let popup_area = self.centered_rect(70, 40, f.area());
        f.render_widget(Clear, popup_area);

        let local_block = Block::default()
            .title("Local Storage Configuration")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        let fields = vec![
            format!("Path: {}", self.temp_config.local_config.path),
        ];

        let items: Vec<ListItem> = fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let style = if i == self.config_field_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(field.as_str()).style(style)
            })
            .collect();

        let local_list = List::new(items)
            .block(local_block)
            .highlight_style(Style::default().bg(Color::Blue));

        f.render_widget(local_list, popup_area);

        self.render_instructions(f, popup_area, "Enter: Edit, S: Save & Back, Esc: Back");
    }

    fn render_mongodb_config(&self, f: &mut Frame) {
        let popup_area = self.centered_rect(80, 50, f.area());
        f.render_widget(Clear, popup_area);

        let mongo_block = Block::default()
            .title("MongoDB Configuration")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        let fields = vec![
            format!("Connection URL: {}", self.temp_config.mongo_config.connection_string),
            format!("Database: {}", self.temp_config.mongo_config.database),
            format!("Collection: {}", self.temp_config.mongo_config.collection),
        ];

        let items: Vec<ListItem> = fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let style = if i == self.config_field_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(field.as_str()).style(style)
            })
            .collect();

        let mongo_list = List::new(items)
            .block(mongo_block)
            .highlight_style(Style::default().bg(Color::Blue));

        f.render_widget(mongo_list, popup_area);

        self.render_instructions(f, popup_area, "↑/↓: Navigate, Enter: Edit, S: Save & Back, Esc: Back");
    }

    fn render_instructions(&self, f: &mut Frame, popup_area: ratatui::layout::Rect, text: &str) {
        let instructions_area = ratatui::layout::Rect {
            x: popup_area.x,
            y: popup_area.y + popup_area.height - 3,
            width: popup_area.width,
            height: 3,
        };

        let instructions = Paragraph::new(text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);

        f.render_widget(instructions, instructions_area);
    }

    fn render_notification(&self, f: &mut Frame, notification: &Notification) {
        let area = f.area();
        let notification_width = 40;
        let notification_height = 3;
        
        let notification_area = ratatui::layout::Rect {
            x: area.width.saturating_sub(notification_width + 2),
            y: 1,
            width: notification_width,
            height: notification_height,
        };

        f.render_widget(Clear, notification_area);

        let (style, border_style) = match notification.level {
            NotificationLevel::Success => (
                Style::default().fg(Color::White).bg(Color::Green),
                Style::default().fg(Color::Green)
            ),
            NotificationLevel::Error => (
                Style::default().fg(Color::White).bg(Color::Red),
                Style::default().fg(Color::Red)
            ),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .style(border_style);

        let paragraph = Paragraph::new(notification.message.as_str())
            .block(block)
            .style(style)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, notification_area);
    }
}