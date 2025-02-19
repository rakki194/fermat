use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub text: String,
    pub key: KeyCode,
    pub is_pressed: bool,
}

impl Button {
    fn new(text: &str, key: KeyCode) -> Self {
        Self {
            text: text.to_string(),
            key,
            is_pressed: false,
        }
    }
}

pub struct ButtonGrid {
    buttons: Vec<Button>,
    selected: Option<usize>,
    last_clicked_button: Option<usize>,
}

impl ButtonGrid {
    pub fn new() -> Self {
        let buttons = vec![
            // Row 1: Clear buttons and parentheses
            Button::new("C", KeyCode::Char('c')),
            Button::new("CE", KeyCode::Char('e')),
            Button::new("(", KeyCode::Char('(')),
            Button::new(")", KeyCode::Char(')')),
            // Row 2: Advanced operations
            Button::new("sqrt", KeyCode::Char('s')),
            Button::new("abs", KeyCode::Char('a')),
            Button::new("^", KeyCode::Char('^')),
            Button::new("%", KeyCode::Char('%')),
            // Row 3: Numbers 7-9 and division
            Button::new("7", KeyCode::Char('7')),
            Button::new("8", KeyCode::Char('8')),
            Button::new("9", KeyCode::Char('9')),
            Button::new("/", KeyCode::Char('/')),
            // Row 4: Numbers 4-6 and multiplication
            Button::new("4", KeyCode::Char('4')),
            Button::new("5", KeyCode::Char('5')),
            Button::new("6", KeyCode::Char('6')),
            Button::new("*", KeyCode::Char('*')),
            // Row 5: Numbers 1-3 and subtraction
            Button::new("1", KeyCode::Char('1')),
            Button::new("2", KeyCode::Char('2')),
            Button::new("3", KeyCode::Char('3')),
            Button::new("-", KeyCode::Char('-')),
            // Row 6: Zero, decimal, factorial, and addition
            Button::new("0", KeyCode::Char('0')),
            Button::new(".", KeyCode::Char('.')),
            Button::new("!", KeyCode::Char('!')),
            Button::new("+", KeyCode::Char('+')),
        ];

        Self {
            buttons,
            selected: None,
            last_clicked_button: None,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<String> {
        let key_code = key.code;
        for (idx, button) in self.buttons.iter_mut().enumerate() {
            if button.key == key_code {
                button.is_pressed = true;
                self.last_clicked_button = Some(idx);
                let result = match button.text.as_str() {
                    "C" => "CLEAR_ALL".to_string(),
                    "CE" => "CLEAR_ENTRY".to_string(),
                    "sqrt" => "sqrt(".to_string(),
                    "abs" => "abs(".to_string(),
                    _ => button.text.clone(),
                };
                // Reset button state immediately
                button.is_pressed = false;
                self.last_clicked_button = None;
                return Some(result);
            }
        }
        None
    }

    pub fn handle_mouse_event(&mut self, mouse: MouseEvent, area: Rect) -> Option<String> {
        let x = mouse.column as u16;
        let y = mouse.row as u16;

        if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
            let relative_x = (x - area.x) as usize;
            let relative_y = (y - area.y) as usize;

            let cols = 4;
            let button_width = area.width as usize / cols;
            let button_height = 3;

            let col = relative_x / button_width;
            let row = relative_y / button_height;

            let index = row * cols + col;
            if index < self.buttons.len() {
                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        // Only set the button as pressed
                        self.buttons[index].is_pressed = true;
                        self.last_clicked_button = Some(index);
                        None
                    }
                    MouseEventKind::Up(MouseButton::Left) => {
                        // Reset button state
                        if let Some(last_idx) = self.last_clicked_button {
                            self.buttons[last_idx].is_pressed = false;
                        }
                        self.last_clicked_button = None;

                        // Only return the result if we're still over the same button
                        if let Some(last_idx) = self.last_clicked_button {
                            if last_idx == index {
                                let result = match self.buttons[index].text.as_str() {
                                    "C" => "CLEAR_ALL".to_string(),
                                    "CE" => "CLEAR_ENTRY".to_string(),
                                    "sqrt" => "sqrt(".to_string(),
                                    "abs" => "abs(".to_string(),
                                    _ => self.buttons[index].text.clone(),
                                };
                                Some(result)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        // Update which button is pressed when dragging
                        if let Some(last_idx) = self.last_clicked_button {
                            if last_idx != index {
                                self.buttons[last_idx].is_pressed = false;
                                self.buttons[index].is_pressed = true;
                                self.last_clicked_button = Some(index);
                            }
                        }
                        None
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else if mouse.kind == MouseEventKind::Up(MouseButton::Left) {
            // Reset button state when mouse is released outside
            if let Some(last_idx) = self.last_clicked_button {
                self.buttons[last_idx].is_pressed = false;
                self.last_clicked_button = None;
            }
            None
        } else {
            None
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let button_style = Style::default().bg(Color::DarkGray).fg(Color::White);
        let pressed_style = Style::default()
            .bg(Color::LightBlue) // Changed to light blue for better visibility
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD);

        // Create a 4x6 grid layout
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3), // Row height
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        for (row_idx, row) in rows.iter().enumerate() {
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .split(*row);

            for (col_idx, col) in cols.iter().enumerate() {
                let button_idx = row_idx * 4 + col_idx;
                if button_idx < self.buttons.len() {
                    let button = &self.buttons[button_idx];
                    let style = if button.is_pressed {
                        pressed_style
                    } else {
                        button_style
                    };

                    let button_widget = Paragraph::new(button.text.clone())
                        .style(style)
                        .block(Block::default().borders(Borders::ALL))
                        .alignment(Alignment::Center);

                    frame.render_widget(button_widget, *col);
                }
            }
        }
    }
}
