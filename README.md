# Rust: Summer Internship
<details>
<summary>Day-01: Installation And Introduction to Variables & Datatypes</summary>

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
<details>
<summary>Day-02: Compound Datatypes And Control Flow</summary>

## Tuple

### Declaration
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

### Accessing Tuple
```rust
tup.0 
tup.1
```

- Tuples have a fixed length and cannot grow or shrink.

### Destructuring a Tuple
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

### Unit
- The tuple without any values has a special name, unit. This value and its corresponding type are both written `()` and represent an empty value or an empty return type.

### Memory Allocation
- Tuples are allocated on the stack, not on the heap.

## Array

### Properties
- Unlike a tuple, every element of an array must have the same type.
- Arrays have a fixed length.

### Declaration
```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // To declare an array with the same elements.
```

### Accessing Arrays
```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

### Memory Allocation
- Arrays are allocated on the stack.

### Invalid Accessing of Array
```rust
let a = [1, 2, 3, 4, 5];
let first = a[5]; // Error: Index out of bounds
```


# Control Flow

## If Expression
```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

## Multiple Conditions with else if
```rust
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

## Using if in let Statement
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```


</details>