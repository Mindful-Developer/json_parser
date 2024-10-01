# JSON Parser

A robust JSON parser implemented in Rust, capable of parsing JSON objects and arrays with various data types.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Overview

This JSON parser is a Rust implementation that adheres to the JSON specification. It can parse JSON objects and arrays, handling various data types including strings, numbers, booleans, null values, and nested structures.

## Features

- Parse JSON objects and arrays
- Support for string, number, boolean, and null values
- Nested object and array parsing
- Robust error handling with detailed error messages
- Comprehensive test suite covering various JSON scenarios

## Project Structure

```
json_parser/
├── src/
│   ├── language_tools/
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── tokens.rs
│   │   └── mod.rs
│   ├── tests/
│   │   └── (various JSON test files)
│   ├── main.rs
│   └── tests.rs
├── Cargo.toml
└── .gitignore
```

- `language_tools/`: Contains the core parsing logic
    - `lexer.rs`: Tokenizes the input JSON string
    - `parser.rs`: Parses the tokens into a JSON structure
    - `tokens.rs`: Defines token types and positions
- `tests/`: Contains JSON files for testing various scenarios
- `main.rs`: Entry point for the command-line interface
- `tests.rs`: Contains the test suite

## Getting Started

1. Ensure you have Rust installed on your system. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone the repository:
   ```
   git clone https://github.com/yourusername/json_parser.git
   cd json_parser
   ```

3. Build the project:
   ```
   cargo build
   ```

## Usage

To use the JSON parser as a command-line tool:

```
cargo run -- <path_to_json_file>
```

The parser will validate the JSON file and exit with status code 0 if valid, or 1 if invalid (with an error message).

To use the parser in your Rust code:

```rust
use json_parser::language_tools::parser::Parser;

fn main() {
    let json_str = r#"{"key": "value", "number": 42}"#;
    let mut parser = Parser::new(json_str);
    match parser.parse() {
        Ok(json_value) => println!("Parsed JSON: {:?}", json_value),
        Err(e) => eprintln!("Error parsing JSON: {}", e),
    }
}
```

## Testing

The project includes a comprehensive test suite. To run all tests:

```
cargo test
```

The tests cover various scenarios, including:
- Empty objects and arrays
- String parsing with escape characters
- Number parsing (integers, floats, scientific notation)
- Boolean and null values
- Nested objects and arrays
- Error cases (invalid syntax, unterminated strings, etc.)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open-source and available under the MIT License.