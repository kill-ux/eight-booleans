# Eight-Booleans

A Rust library for managing 8 boolean values (bits) in a single byte using bitwise operators. This project demonstrates efficient bit manipulation through a clean, type-safe API.

## Overview

`ByteBool` is a lightweight data structure that packs 8 independent boolean values into a single `u8` (byte), allowing you to store and manipulate boolean flags with minimal memory overhead. Each bit in the byte represents one boolean value.

## Features

- **Compact Storage**: Store 8 booleans in just 1 byte (8 bits)
- **Type-Safe Index**: Uses a custom `Index` type to prevent out-of-range access at compile time
- **Efficient Operations**: Uses bitwise operators for O(1) operations
- **Simple API**: Clear, intuitive methods for common bit operations

## Supported Operations

### SET
Set a specific bit to `true` (1) or `false` (0):
```rust
let mut bits = ByteBool::new();
bits.set(0, true);   // Set bit 0 to true
bits.set(3, false);  // Set bit 3 to false
```

### READ
Read the value of a specific bit:
```rust
let bit_value = bits.read(0);  // Returns true or false
```

### TOGGLE
Flip a bit from 1 to 0 or 0 to 1:
```rust
bits.toggle(5);  // Flip bit 5
```

### CLEAR
Reset all bits to 0:
```rust
bits.clear();  // Set all 8 bits to false
```

### DISPLAY
Visualize the current state as binary:
```rust
bits.display();  // Prints: 00000000 (or current state)
```

## Index Type

The `Index` struct ensures type-safe bit indexing:
- Valid range: 0-7 (for 8 bits)
- Construction:
  - `Index::new(value)` - Returns `Result<Index, String>` (safe, checked)
  - `Index::new_unchecked(value)` - Panics if out of range
  - `u8::into()` - Converts u8 directly using `From` trait

## Usage Example

```rust
use eight_booleans::ByteBool;

fn main() {
    let mut bits = ByteBool::new();
    
    // Set some bits
    bits.set(0, true);
    bits.set(3, true);
    bits.set(7, true);
    bits.display();  // Output: 10001001
    
    // Read a bit
    let is_set = bits.read(0);  // true
    
    // Toggle a bit
    bits.toggle(0);
    bits.display();  // Output: 10001000
    
    // Clear all
    bits.clear();
    bits.display();  // Output: 00000000
}
```

## Building and Running

### Run the example:
```bash
cargo run
```

### Build the library:
```bash
cargo build
```

### Run tests:
```bash
cargo test
```

## Project Structure

```
.
├── Cargo.toml           # Project manifest
├── README.md            # This file
└── src/
    ├── lib.rs           # Core library (Index, ByteBool)
    └── main.rs          # Example usage
```

## Implementation Details

- **Index Validation**: The `Index` type prevents invalid indices at compile time
- **Bitwise Operations**:
  - `SET`: Uses OR (`|`) and AND with NOT (`&!`)
  - `READ`: Uses right shift (`>>`) and AND (`&`)
  - `TOGGLE`: Uses XOR (`^`)
  - `CLEAR`: Direct assignment to 0
- **Memory Efficient**: `ByteBool` is a `Copy` type, enabling efficient passing and manipulation

## API Reference

### ByteBool

```rust
pub fn new() -> Self                                // Create new (all bits 0)
pub fn clear(&mut self)                             // Set all bits to 0
pub fn read(self, index: u8) -> bool                // Read a bit
pub fn set(&mut self, index: u8, new_value: bool)   // Set a bit
pub fn toggle(&mut self, index: u8)                 // Flip a bit
pub fn display(self)                                // Print binary representation
```

### Index

```rust
pub fn new(value: u8) -> Self                      // Unchecked construction (panics if the )
pub fn get(&self) -> u8                            // Get inner value
```

## License

This project is open source and available under the MIT License.
