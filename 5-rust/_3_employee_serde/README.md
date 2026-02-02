# Employee Serde 

 A Rust project demonstrating structured data handling with nested structs and JSON serialization/deserialization using Serde.

## Features

- Nested `Employee` and `Address` structs
- Getters, setters, and nested field updates
- Display employee info and greetings
- JSON serialization and deserialization

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
