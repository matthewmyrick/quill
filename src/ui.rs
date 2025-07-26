use crate::storage::{Todo, TodoStatus};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub struct TodoUI {
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input_text: String,
    pub editing_id: Option<usize>,
}

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Adding,
    Editing,
}

impl Default for TodoUI {
    fn default() -> Self {
        Self {
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            input_text: String::new(),
            editing_id: None,
        }
    }
}

impl TodoUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_next(&mut self, todos: &[Todo]) {
        if todos.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let next = if selected >= todos.len() - 1 {
            0
        } else {
            selected + 1
        };
        self.list_state.select(Some(next));
    }

    pub fn select_previous(&mut self, todos: &[Todo]) {
        if todos.is_empty() {
            return;
        }
        
        let selected = self.list_state.selected().unwrap_or(0);
        let previous = if selected == 0 {
            todos.len() - 1
        } else {
            selected - 1
        };
        self.list_state.select(Some(previous));
    }

    pub fn start_adding(&mut self) {
        self.input_mode = InputMode::Adding;
        self.input_text.clear();
    }

    pub fn start_editing(&mut self, todo: &Todo) {
        self.input_mode = InputMode::Editing;
        self.input_text = todo.text.clone();
        self.editing_id = Some(todo.id);
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

    pub fn render(&mut self, f: &mut Frame, todos: &[Todo], context: &str) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Header
        let header = Paragraph::new(format!("Quill Todo - {}", context))
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Todo List
        let items: Vec<ListItem> = todos
            .iter()
            .map(|todo| {
                let (symbol, style) = match todo.status {
                    TodoStatus::NotStarted => ("○", Style::default().fg(Color::Yellow)),
                    TodoStatus::InProgress => ("◐", Style::default().fg(Color::Blue)),
                    TodoStatus::Completed => ("✓", Style::default().fg(Color::Green).add_modifier(Modifier::CROSSED_OUT)),
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!("{} ", symbol), style),
                    Span::styled(&todo.text, style),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Todos"))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("➤ ");

        f.render_stateful_widget(list, chunks[1], &mut self.list_state);

        // Footer
        let footer_text = "Press 'a' to add, 'e' to edit (not completed), 'd' to delete, Space to cycle status, '1'=Not Started, '2'=In Progress, '3'=Completed, 'q' to quit";
        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(footer, chunks[2]);

        // Floating input box
        if self.input_mode != InputMode::Normal {
            let popup_area = self.centered_rect(60, 20, f.size());
            f.render_widget(Clear, popup_area);
            
            let title = match self.input_mode {
                InputMode::Adding => "Add New Todo",
                InputMode::Editing => "Edit Todo",
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
            f.set_cursor(
                popup_area.x + self.input_text.len() as u16 + 1,
                popup_area.y + 1,
            );
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
}