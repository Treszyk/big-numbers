# big-numbers

`big-numbers` is a pure-Rust library for arbitrary-precision integer arithmetic. It provides robust support for both unsigned (`BigUInt`) and signed (`BigInt`) integers of any size, limited only by your system's memory.

## Why big-numbers

- **Zero Dependencies**: A lightweight implementation that relies solely on the Rust standard library.
- **Limb-based Architecture**: Uses a base $2^{32}$ (u32 limbs) representation for efficient storage and computation.
- **Full Arithmetic Suite**: Complete support for addition, subtraction, multiplication, and division.
- **Safe Signed Integers**: `BigInt` implementation using sign-magnitude representation.
- **Seamless Parsing**: Built-in support for string parsing and display in base 10.

## How it works (high level)

1. **Storage**: Numbers are stored as a `Vec<u32>` of limbs, representing the number in base $2^{32}$.
2. **BigUInt**: The core engine that handles unsigned arithmetic. Subtraction includes safety checks for underflow.
3. **BigInt**: A high-level wrapper that combines a `BigUInt` magnitude with a `Sign` enum (Plus/Minus).
4. **Operations**: Most operations are implemented using standard algorithms, ensuring predictable performance for large numbers.

## What’s in this repo

- **src/big_uint.rs** – The core logic for unsigned arbitrary-precision integers.
- **src/big_int.rs** – Support for signed integers and sign-related logic.
- **tests/** – Comprehensive integration tests for both signed and unsigned types.
- **examples/galactic_test.rs** – A performance benchmark demonstrating large-scale multiplication (~2.1M bits).

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed.

### Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Treszyk/big-numbers.git
   cd big-numbers
   ```

2. **Run Tests**:
   ```bash
   cargo test
   ```

3. **Run Performance Example**:
   ```bash
   cargo run --example galactic_test --release
   ```

## Roadmap

This project is under active development. Planned improvements include:

- [ ] **Optimized Math**: Implementing more efficient algorithms for multiplication (e.g., Karatsuba) and division.
- [ ] **Fixed-Point Arithmetic**: Support for high-precision decimal calculations.
- [ ] **Floating-Point Arithmetic**: Comprehensive IEEE 754-like support for arbitrary-precision floats.
- [ ] **Bitwise Operations**: Efficient bit-level manipulation for `BigUInt`.

## License

This project is licensed under the [MIT License](LICENSE).
