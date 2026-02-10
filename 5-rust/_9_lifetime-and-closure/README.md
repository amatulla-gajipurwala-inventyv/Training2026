# Generic Inventory System in Rust

This project demonstrates a generic inventory management system in Rust using traits, lifetimes, and explicit error handling. Items are stored as borrowed references to avoid unnecessary cloning and to highlight safe lifetime management.

---

## Overview

The inventory uses a `HashMap<String, &T>` to store items by unique ID.  
All item types must implement a common trait to ensure consistent display behavior.

Key concepts:
- Generics and trait bounds
- Lifetime-based reference storage
- Safe collection mutation with `HashMap::entry`
- Structured error handling using `Result`

---

## Core Abstractions

- **DisplayItem trait** – defines a standard `display()` interface for items  
- **Inventory<'a, T>** – generic container storing borrowed items with lifetime guarantees  
- **InventoryError** – handles duplicate IDs, invalid IDs, and missing items

---

## Functionality

- Add items with validation and duplicate checks
- Retrieve items by ID
- Remove items safely
- Display inventory using a caller-provided formatter

---

## Sample Output

```text
Inventory:
Inventory is empty

Inventory:
ID: P2
Product: Neckband, Price: ₹1500

ID: P1
Product: Laptop, Price: ₹60000

Fetched: Product: Neckband, Price: ₹1500
Removed: Product: Laptop, Price: ₹60000

Inventory:
ID: P2
Product: Neckband, Price: ₹1500
```
## Run
```rust
cargo run
```
