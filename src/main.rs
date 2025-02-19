//! Main module for the Calculator TUI application.
//!
//! This module initializes the terminal UI, handles user input in real-time, and updates
//! the display with the current expression and its evaluated result. The evaluation is
//! performed automatically as the user types.

mod button_grid;
mod evaluator;

use crossterm::{
    ExecutableCommand,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use rust_decimal::prelude::*;
use std::io; // Removed unused stdout

/// A simple calculator structure that holds the current input expression and its evaluated result.
struct Calculator {
    /// The current input expression as a string.
    input: String,
    /// The result of evaluating the input expression. None if the input is empty or invalid.
    result: Option<String>,
    button_grid: button_grid::ButtonGrid,
    max_input_length: usize, // Add maximum input length
}

impl Calculator {
    /// Creates a new Calculator instance with empty input and no result.
    fn new() -> Self {
        Self {
            input: String::new(),
            result: None,
            button_grid: button_grid::ButtonGrid::new(),
            max_input_length: 50, // Reasonable limit for input length
        }
    }

    /// Handles a key press event and automatically re-evaluates the expression.
    ///
    /// Accepts digits, operators, and special characters. Backspace removes the last character.
    /// After processing the key, it updates the evaluated result automatically.
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') if self.input.is_empty() => {
                return;
            }
            _ => {
                if let Some(text) = self
                    .button_grid
                    .handle_key_event(event::KeyEvent::new(key, event::KeyModifiers::empty()))
                {
                    match text.as_str() {
                        "CLEAR_ALL" => {
                            self.input.clear();
                        }
                        "CLEAR_ENTRY" => {
                            // Remove the last number or operation
                            while let Some(c) = self.input.chars().last() {
                                if c.is_whitespace() {
                                    break;
                                }
                                self.input.pop();
                            }
                        }
                        _ => {
                            // Check if adding the text would exceed the maximum length
                            if self.input.len() + text.len() <= self.max_input_length {
                                self.input.push_str(&text);
                            } else {
                                self.result = Some("Error: Input too long".to_string());
                                return;
                            }
                        }
                    }
                } else {
                    match key {
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        _ => {}
                    }
                }
            }
        }
        self.evaluate();
    }

    /// Evaluates the current input expression and updates the result field.
    ///
    /// If the input is empty, the result is set to None. Otherwise, the input is tokenized and evaluated.
    /// Any error during evaluation will be captured and stored as the result string.
    fn evaluate(&mut self) {
        if self.input.is_empty() {
            self.result = None;
            return;
        }

        // Parse individual numbers and check if they're within reasonable bounds
        let mut current_number = String::new();
        for c in self.input.chars() {
            if c.is_ascii_digit() || c == '.' {
                current_number.push(c);
            } else if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    if !num.is_finite() || num.abs() > 1e100 {
                        self.result = Some("Error: Number too large".to_string());
                        return;
                    }
                }
                current_number.clear();
            }
        }
        // Check the last number if exists
        if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<f64>() {
                if !num.is_finite() || num.abs() > 1e100 {
                    self.result = Some("Error: Number too large".to_string());
                    return;
                }
            }
        }

        match evaluator::tokenize(&self.input) {
            Ok(tokens) => {
                match evaluator::evaluate(&tokens) {
                    Ok(result) => {
                        // Check if the result is too large
                        if result > Decimal::from_str("1e50").unwrap_or(Decimal::MAX)
                            || result < Decimal::from_str("-1e50").unwrap_or(Decimal::MIN)
                        {
                            self.result = Some("Error: Result too large".to_string());
                            return;
                        }
                        // Format the result to prevent excessive decimal places
                        let result_str = format!("{:.10}", result);
                        // Remove trailing zeros after decimal point
                        let result_str = if result_str.contains('.') {
                            result_str
                                .trim_end_matches('0')
                                .trim_end_matches('.')
                                .to_string()
                        } else {
                            result_str
                        };
                        self.result = Some(result_str);
                    }
                    Err(e) => {
                        self.result = Some(format!("Error: {}", e));
                    }
                }
            }
            Err(e) => {
                self.result = Some(format!("Error: {}", e));
            }
        }
    }
}

/// The main function that sets up the terminal, runs the TUI loop and cleans up on exit.
fn main() -> io::Result<()> {
    // Setup terminal in raw mode and switch to alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(EnableMouseCapture)?;
    stdout.execute(Clear(ClearType::All))?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut calculator = Calculator::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            // Create the layout with designated areas for input and result
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3), // Input area
                        Constraint::Length(3), // Result area
                        Constraint::Min(20),   // Button grid area
                    ]
                    .as_ref(),
                )
                .split(area);

            // Render input field with character count
            let input_block = Block::default().borders(Borders::ALL).title(format!(
                "Input ({}/{})",
                calculator.input.len(),
                calculator.max_input_length
            ));
            let input = Paragraph::new(calculator.input.as_str()).block(input_block);
            frame.render_widget(input, chunks[0]);

            // Render result field
            let result_block = Block::default().borders(Borders::ALL).title("Result");
            let result =
                Paragraph::new(calculator.result.as_deref().unwrap_or("")).block(result_block);
            frame.render_widget(result, chunks[1]);

            // Render button grid
            calculator.button_grid.render(frame, chunks[2]);
        })?;

        match event::read()? {
            Event::Key(key) => {
                if key.kind == event::KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') && calculator.input.is_empty() {
                        break;
                    }
                    calculator.handle_key(key.code);
                }
            }
            Event::Mouse(mouse) => {
                if let Some(text) = calculator
                    .button_grid
                    .handle_mouse_event(mouse, terminal.get_frame().area())
                {
                    match text.as_str() {
                        "CLEAR_ALL" => {
                            calculator.input.clear();
                        }
                        "CLEAR_ENTRY" => {
                            // Remove the last number or operation
                            while let Some(c) = calculator.input.chars().last() {
                                if c.is_whitespace() {
                                    break;
                                }
                                calculator.input.pop();
                            }
                        }
                        _ => {
                            // Check if adding the text would exceed the maximum length
                            if calculator.input.len() + text.len() <= calculator.max_input_length {
                                calculator.input.push_str(&text);
                            } else {
                                calculator.result = Some("Error: Input too long".to_string());
                                continue;
                            }
                        }
                    }
                    calculator.evaluate();
                }
            }
            _ => {}
        }
    }

    // Cleanup terminal state on exit
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    io::stdout().execute(DisableMouseCapture)?;
    Ok(())
}
