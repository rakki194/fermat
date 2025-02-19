# Fermat Calculator

A powerful Terminal User Interface (TUI) calculator built in Rust that provides advanced mathematical operations with high-precision decimal arithmetic.

## Features

- 🖥️ Clean and intuitive Terminal User Interface
- 🔢 High-precision decimal arithmetic using `rust_decimal`
- ➗ Comprehensive set of mathematical operations:
  - Basic arithmetic: `+`, `-`, `*`, `/`
  - Advanced operations:
    - Exponentiation (`^`)
    - Modulo (`%`)
    - Factorial (`!`)
    - Square root (`sqrt()`)
    - Absolute value (`abs()`)
- 📐 Proper operator precedence handling
- 🎯 Real-time expression evaluation
- 🔄 Support for parentheses and nested expressions
- ⚡ Efficient parsing using the `nom` parser combinator library
- 🎨 Beautiful TUI powered by `ratatui` and `crossterm`

## Installation

### Prerequisites

- Rust toolchain (2024 edition)
- Cargo package manager

### Building from Source

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/fermat.git
    cd fermat
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

The compiled binary will be available at `target/release/calculator`

## Usage

Run the calculator:

```bash
cargo run --release
```

### Input Format

- Type mathematical expressions using the supported operators
- Press Enter to evaluate (evaluation also happens in real-time)
- Press 'q' to quit when the input field is empty

### Examples

```r
2 + 2             # Basic arithmetic
(3 + 4) * 2       # Parentheses for grouping
2^3               # Exponentiation
5!                # Factorial
sqrt(16)          # Square root
abs(-5)           # Absolute value
```

### Keyboard Controls

- `0-9`: Input numbers
- `+-*/^%`: Arithmetic operators
- `()`: Parentheses
- `!`: Factorial
- `s`: Insert `sqrt(`
- `a`: Insert `abs(`
- `Backspace`: Delete last character
- `q`: Quit (when input is empty)

## Technical Details

### Architecture

The calculator is built with a clean separation of concerns:

- `main.rs`: TUI setup and input handling
- `evaluator.rs`: Expression parsing and evaluation
- `lib.rs`: Library interface

### Dependencies

- `ratatui`: Terminal user interface framework
- `crossterm`: Terminal manipulation
- `nom`: Parser combinator library
- `rust_decimal`: High-precision decimal arithmetic
- `rust_decimal_macros`: Decimal literals support

### Mathematical Features

- Supports high-precision decimal arithmetic
- Handles unary minus operations
- Proper operator precedence
- Special case optimization for certain mathematical patterns
- Robust error handling for invalid expressions

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust 🦀
- Powered by `ratatui`, `crossterm`, and `nom`
- Uses `rust_decimal` for precise arithmetic
