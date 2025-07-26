use crate::{git::GitContext, storage::{TodoStorage, TodoStatus}, ui::{InputMode, TodoUI}};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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
    ui: TodoUI,
    storage: TodoStorage,
    current_context: GitContext,
    last_context_check: Instant,
}

impl App {
    pub async fn new() -> Result<Self> {
        let storage = TodoStorage::load()?;
        let current_context = GitContext::from_current_dir()?;
        
        Ok(Self {
            ui: TodoUI::new(),
            storage,
            current_context,
            last_context_check: Instant::now(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

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

            let todos = self.storage.get_todos(&self.current_context.context_key());
            
            terminal.draw(|f| {
                self.ui.render(f, &todos, &self.current_context.context_key());
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match self.ui.input_mode {
                            InputMode::Normal => {
                                if self.handle_normal_input(key.code).await? {
                                    break;
                                }
                            }
                            InputMode::Adding | InputMode::Editing => {
                                self.handle_input_mode(key.code).await?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_normal_input(&mut self, key: KeyCode) -> Result<bool> {
        let todos = self.storage.get_todos(&self.current_context.context_key());
        
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('a') => {
                self.ui.start_adding();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.ui.select_next(&todos);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.ui.select_previous(&todos);
            }
            KeyCode::Char(' ') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(todo) = todos.get(selected) {
                        self.storage.toggle_todo(&self.current_context.context_key(), todo.id)?;
                    }
                }
            }
            KeyCode::Char('1') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(todo) = todos.get(selected) {
                        self.storage.set_todo_status(&self.current_context.context_key(), todo.id, TodoStatus::NotStarted)?;
                    }
                }
            }
            KeyCode::Char('2') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(todo) = todos.get(selected) {
                        self.storage.set_todo_status(&self.current_context.context_key(), todo.id, TodoStatus::InProgress)?;
                    }
                }
            }
            KeyCode::Char('3') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(todo) = todos.get(selected) {
                        self.storage.set_todo_status(&self.current_context.context_key(), todo.id, TodoStatus::Completed)?;
                    }
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(todo) = todos.get(selected) {
                        self.storage.remove_todo(&self.current_context.context_key(), todo.id)?;
                        if selected > 0 && selected >= todos.len() - 1 {
                            self.ui.list_state.select(Some(selected - 1));
                        }
                    }
                }
            }
            KeyCode::Char('e') => {
                if let Some(selected) = self.ui.list_state.selected() {
                    if let Some(todo) = todos.get(selected) {
                        // Don't allow editing completed todos
                        if !matches!(todo.status, TodoStatus::Completed) {
                            self.ui.start_editing(todo);
                        }
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
                            self.storage.edit_todo(&self.current_context.context_key(), id, text)?;
                        }
                        None => {
                            self.storage.add_todo(&self.current_context.context_key(), text)?;
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
}