# Rust HashMap & HashSet Operations Demo

This project demonstrates practical usage of Rust’s `HashMap` and `HashSet` from `std::collections`.  
It covers **insertion, overwriting behavior, cloning, capacity management, safe removal, extension, and filtering** using a custom `User` struct.

---

## Data Model

```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct User {
    id: u32,
    name: String,
}
```
###  notes:

   - Eq and Hash enable storage in HashSet

   - Clone allows explicit value copying

   - Debug supports structured logging and output

### HashMap Highlights

   - Keys are unique; inserting an existing key replaces the value

   - clone() performs a deep copy of the map

   - len() reports stored entries, while capacity() reflects allocated memory

   - try_reserve() pre-allocates space to reduce rehashing

   - Values are safely extracted using Option and take()

   - remove() deletes a key-value pair entirely

   - extend() merges another map and transfers ownership

   - retain() filters entries based on a predicate

### HashSet Highlights

   - Only unique values are stored, based on Eq and Hash

   - Supports deep cloning via clone()

   - Capacity management mirrors HashMap

   - take() removes and returns a matching value

   - extend() merges sets, ignoring duplicates

   - retain() removes elements that do not match a condition

### Build and Run
```rust
cargo run
```