# Rust: Summer Internship
<details>
<summary>Day-01</summary>

## Installation

### For macOS
To install Rust on macOS, use the following command:
```sh
curl -proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |sh
```
```sh
source $HOME/.cargo/env
```

### For Windows
Download and install Rust from the official website:
[Rust Installation for Windows](https://www.rust-lang.org/tools/install)

## Verifying Installation
To verify the installation, run:
```sh
rustc --version
```

## Creating a New Project

### To create a project in the same folder:
```sh
cargo init
```

### To create a completely new project:
```sh
cargo new demo-project
```

## Unique Features of Rust

- **Memory Safety**
- **Ownership and Borrowing**
- **Concurrency**
- **Pattern Matching**
- **Cargo**

## Advantages Over Other Languages

- Rust guarantees memory safety without a garbage collector, reducing the risk of crashes and security vulnerabilities.
- Rust's performance is comparable to C and C++, making it suitable for high-performance applications.
- Rust's unique approach to concurrency helps avoid data races, which are common in languages like C++ and Java.
- Rust has a strong and growing community with a rich ecosystem of libraries and tools, making it easier to find support and resources.
- Rust's compiler and tools like Cargo provide excellent support for development, testing, and deployment, enhancing productivity.

## Variables

- Declare variables using `let`.
- By default, variables are immutable.
- Declare mutable variables by adding `mut` before the variable names.
- Constants are similar to immutable variables; they are bound to a name and are not allowed to change.
- Declare constants with `const`.

### Example:
```rust
let x = 5;
let mut y = 10;
const MAX_POINTS: u32 = 100_000;
```

## Shadowing

- Reassign a value to a variable.
- Shadowing is different from making a variable `mut`, as it allows reassignment without `let`.
- Shadowing also allows changing the type of a variable, which is not possible with `mut`.

### Example:
```rust
let x = 5;
let x = x + 1;
let x = "shadowed";
```

## Data Types

Rust is a statically typed language, meaning the types of all variables are known and checked at compile time.

### Integer Types
- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (represent both positive and negative numbers)
  - Example: `i8` has a range from -128 to 127
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (represent only non-negative numbers)
  - Example: `u8` has a range from 0 to 255

### Boolean Type
```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

### Character Type
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
}
```

</details>