//! Main module for the Calculator TUI application.
//!
//! This module initializes the terminal UI, handles user input in real-time, and updates
//! the display with the current expression and its evaluated result. The evaluation is
//! performed automatically as the user types.

mod evaluator;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io; // Removed unused stdout

/// A simple calculator structure that holds the current input expression and its evaluated result.
struct Calculator {
    /// The current input expression as a string.
    input: String,
    /// The result of evaluating the input expression. None if the input is empty or invalid.
    result: Option<String>,
}

impl Calculator {
    /// Creates a new Calculator instance with empty input and no result.
    fn new() -> Self {
        Self {
            input: String::new(),
            result: None,
        }
    }

    /// Handles a key press event and automatically re-evaluates the expression.
    ///
    /// Accepts digits, operators, and special characters. Backspace removes the last character.
    /// After processing the key, it updates the evaluated result automatically.
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') if self.input.is_empty() => {
                // Only quit if input is empty
                return;
            }
            KeyCode::Char('s') => {
                self.input.push_str("sqrt(");
            }
            KeyCode::Char('a') => {
                self.input.push_str("abs(");
            }
            KeyCode::Char(c) => {
                // Allow all valid calculator characters and spaces
                if c.is_ascii_digit() || "+-*/().^%! ".contains(c) {
                    self.input.push(c);
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
        // Automatically evaluate the current input
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
        match evaluator::tokenize(&self.input) {
            Ok(tokens) => {
                match evaluator::evaluate(&tokens) {
                    Ok(result) => {
                        self.result = Some(result.to_string());
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
                .constraints([
                    Constraint::Length(3), // Input area
                    Constraint::Length(3), // Result area
                    Constraint::Min(0),    // Remaining space
                ].as_ref())
                .split(area);

            // Render input field
            let input_block = Block::default()
                .borders(Borders::ALL)
                .title("Input");
            let input = Paragraph::new(calculator.input.as_str())
                .block(input_block);
            frame.render_widget(input, chunks[0]);

            // Render result field
            let result_block = Block::default()
                .borders(Borders::ALL)
                .title("Result");
            let result = Paragraph::new(calculator.result.as_deref().unwrap_or(""))
                .block(result_block);
            frame.render_widget(result, chunks[1]);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {  // Only handle key press events
                if key.code == KeyCode::Char('q') {
                    break;
                }
                calculator.handle_key(key.code);
            }
        }
    }

    // Cleanup terminal state on exit
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

