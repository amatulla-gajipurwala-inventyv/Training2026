# Request Handling in Rust with Mutex and RwLock

A Rust project demonstrating how to handle multiple types of requests while safely tracking statistics using **static counters** with `Mutex` and `RwLock`.

## Overview

This project implements a request processor in Rust with the following features:

- Defines a `Request` enum for different request types:
  - `Get { endpoint: String }`
  - `Post { endpoint: String, payload_size: u32 }`
  - `Delete(u32)` – delete a resource by ID
- Uses static variables to maintain request statistics safely
- Handles requests using a function with `match` for pattern matching
- Demonstrates thread-safe counting using both `Mutex` and `RwLock`

## Features

- Enum-based request representation
- Thread-safe counters for tracking total requests and request types
- Clean pattern matching for handling requests
- Supports printing request responses and updated statistics

### Mutex vs RwLock

| Feature           | `Mutex` | `RwLock` |
|------------------|---------|----------|
| Locks            | Exclusive (one writer/reader at a time) | Allows multiple readers or one writer |
| Use Case         | Updating shared state safely | When reads are frequent but writes are rare |
| Performance      | Lower if many readers | Higher read throughput in read-heavy scenarios |
| Example in this project | `REQUEST_STATS: Mutex<RequestStats>` | `REQUEST_STATS: RwLock<RequestStats>` |

In this project:

- `Mutex` version ensures exclusive access when handling requests.  
- `RwLock` version allows multiple reads simultaneously, improving performance for read-heavy workloads.

## Running the Project

```bash
cargo run
