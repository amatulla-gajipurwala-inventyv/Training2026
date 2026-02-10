# Generic Inventory System in Rust

This repository demonstrates a generic inventory management system in Rust using traits, generics, and structured error handling. The implementation follows idiomatic Rust practices and focuses on type safety, clarity, and reusability.

---

## Overview

The inventory stores items in a `HashMap<String, T>`, where each item is identified by a unique ID.  
All stored types must implement a common trait to ensure consistent display behavior.

The project highlights:
- Generic data structures
- Trait-based abstraction
- Safe ownership and borrowing
- Explicit error handling with `Result`

---

## Core Components

### DisplayItem Trait

```rust
trait DisplayItem {
    fn display(&self) -> String;
}
```
### Inventory Structure
```rust
struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}
```
- Generic over T

- Uses HashMap for efficient lookup

- Enforces compile-time constraints on stored types

### Error Handling

A custom InventoryError enum is used to represent:

- Duplicate item IDs

- Invalid (empty) IDs

- Missing items during lookup or removal

All operations return Result to avoid panics and provide clear failure reasons.

### Supported Operations

- Add Item: Validates ID and prevents duplicates

- Get Item: Fetches an item by ID or returns an error

- Remove Item: Deletes and returns an item safely

- Display Inventory: Formats all items or reports an empty inventory

## Output
```text
Display all : Inventory is empty
Add failed: Item with id 'P1' already exist
Add failed: Invalid Id
 
Current Inventory :
 Id: P1 
Product: Laptop, Price:60000

Id: P2 
Product: Neckband, Price:1500

Fetched item: Product: Neckband, Price:1500

Error: Item with id 'P3' not found
Remove failed: Item with id 'P1' not found
 
Current Inventory :
 Id: P2 
Product: Neckband, Price:1500

```

### Running the Project
  ```rust
  cargo run
```