# Rust Modular Project

This is a Rust project demonstrating modular code design, core Rust concepts, and concurrency. The project is organized into multiple modules, each implementing a specific functionality, all integrated through a single `main.rs`.


---



## Modules Overview

### Loops
- Demonstrates Rust loops: `loop`, `while`, `for`
- Loop returning a value, retry logic, and countdown timers

### Employee Modules
- `employee_struct.rs`: Nested structs, getters/setters, contact methods
- `employee_serde.rs`: JSON serialization/deserialization with Serde
- `employee_mut_ref.rs`: Using mutable references to update structs

### Concurrency Modules
- `mutex_stats.rs`: Tracks requests (`GET`, `POST`, `DELETE`) safely using `Mutex`  
- `rwlock_stats.rs`: Tracks requests using `RwLock` for concurrent reads  