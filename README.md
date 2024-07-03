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
### Array vs Tuple

| Feature                  | Array                                                  | Tuple                                                     |
|--------------------------|--------------------------------------------------------|-----------------------------------------------------------|
| Element Types            | All elements must have the same type                   | Elements can have different types                         |
| Length                   | Fixed length                                           | Fixed length                                              |
| Declaration              | `let a = [1, 2, 3, 4, 5];`                             | `let tup: (i32, f64, u8) = (500, 6.4, 1);`                |
| Type Annotation          | `let a: [i32; 5] = [1, 2, 3, 4, 5];`                   | `let tup: (i32, f64, u8) = (500, 6.4, 1);`                |
| Accessing Elements       | `a[0]`, `a[1]`                                         | `tup.0`, `tup.1`                                          |
| Destructuring            | Not applicable                                         | `let (x, y, z) = tup;`                                    |
| Memory Allocation        | Allocated on stack                                     | Allocated on stack                                        |
| Invalid Access           | Causes a runtime panic (e.g., `a[5]` for an array of 5)| Causes a compile-time error if the tuple index is invalid |
| Use Case                 | Homogeneous collections                                | Heterogeneous collections                                 |
| Empty Representation     | Not applicable                                         | `()` (unit type) 

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

<details>
<summary>Day-03: Loops in Rust</summary>

## Rust Loop Examples
 
Rust has three kinds of loops:
- `loop`
- `for`
- `while`

## Loop

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

Rust also provides a way to break out of a loop using the `break` keyword. You can place the `break` keyword within the loop to tell the program when to stop executing the loop.

You can use `continue` within a loop to skip over any remaining code in this iteration of the loop and go to the next iteration.

### Example of continue statement

```rust
fn main() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("Odd number: {}", i);
    }
}
```

**Output:**
```
Odd number: 1
Odd number: 3
Odd number: 5
Odd number: 7
Odd number: 9
```

### Example of break statement

```rust
fn main() {
    for i in 0..10 {
        if i == 5 {
            break; // Exit the loop when i is 5
        }
        println!("Number: {}", i);
    }
}
```

**Output:**
```
Number: 0
Number: 1
Number: 2
Number: 3
Number: 4
```

### Returning values from loop

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
```

**Output:**
```
The result is 20
```

### Loop labels to Disambiguate Between Multiple loops

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```

**Output:**
```
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
remaining = 9
End count = 2
```

## While

When the condition is true, the loop runs. When the condition ceases to be true, it breaks out of the loop.

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

**Output:**
```
3!
2!
1!
LIFTOFF!!!
```

This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break` and it’s clearer.

## For

We can use a `while` loop to print elements of an array, but if we update the array and forget to change the condition, it will panic at runtime. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}
```

**Output:**
```
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

### Print Elements in Reverse Order

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

**Output:**
```
3!
2!
1!
LIFTOFF!!!
```
</details>

<details>
<summary>Day-04: Matching Statements in Rust</summary>


## Basic Match

- `match` statement is a control flow construct used for pattern matching.
- It allows you to match a value against a series of patterns and execute different code blocks based on the matched pattern.

```rust
fn main() {
    let number = 7;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

## Matching Against Multiple Patterns

- You can match against multiple patterns in a single match statement.

```rust
fn main() {
    let number = 3;

    match number {
        1 | 2 => println!("One or Two"),
        3 | 4 => println!("Three or Four"),
        _ => println!("Other"),
    }
}
```

## Matching with Binding

- You can bind the matched value to a variable, which can be used in the corresponding code block.

```rust
fn main() {
    let number = Some(7);

    match number {
        Some(n) => println!("Found a number: {}", n),
        None => println!("No number found"),
    }
}
```

## Ignoring Values

- If you don't need to use the matched value, you can use the underscore (`_`) pattern to ignore it.

```rust
fn main() {
    let number = Some(7);

    match number {
        Some(_) => println!("Found a number"),
        None => println!("No number found"),
    }
}
```

## Destructuring Tuples

```rust
fn main() {
    let tup = (1, "hello");

    match tup {
        (1, greeting) => println!("The greeting is: {}", greeting),
        _ => println!("No match"),  
    }
}
```

## Matching Ranges

- You can match ranges in match expressions and use `_` to match any value.

```rust
fn main() {
    let age = 21;

    match age {
        1..=17 => println!("minor"),
        18..=120 => println!("major"),
        _ => println!("something else"),
    }
}
```

## Matching Tuples

- Will compare all the elements of tuples and print the appropriate message.

```rust
fn main() {
    let tup = (1, 2, 3, 4);
    match tup {
        (1, 2, 3, 4) => println!("Tuple is 1, 2, 3, 4"),
        _ => println!("Tuple is something else"),
    }
}
```

```rust
fn main() {
    let tup = (1, 2, 3, 4);

    match tup {
        (1, ..) => println!("First element is 1"),
        (_, 2, ..) => println!("Second element is 2"),
        (_, _, 3, ..) => println!("Third element is 3"),
        (.., 4) => println!("Fourth element is 4"),
        (_, _, _, 4) => println!("Fourth element is 4"),
        (x, y, z, a) => println!("{}, {}, {}, {}", x, y, z, a),
        (1, _, _, _) => println!("First element is 1"),
        (_, 2, _, _) => println!("Second element is 2"),
        _ => println!("Tuple is something else"),
    }
}
```

## Differences between `(.., 4)` and `(_, _, _, 4)`

### Differences

1. **Syntax and Readability**:
   - `(.., 4)`: This uses the "rest pattern" (`..`) to indicate that there can be any number of elements before the last one, which must be `4`. This is more flexible and concise.
   - `(_, _, _, 4)`: This explicitly specifies that the tuple must have exactly four elements, with the last one being `4`. This is less flexible but more explicit.

2. **Pattern Matching Flexibility**:
   - `(.., 4)`: This will match any tuple where the last element is `4`, regardless of the number of preceding elements. For example, it matches `(1, 2, 3, 4)`, `(5, 4)`, or `(6, 7, 8, 9, 4)`.
   - `(_, _, _, 4)`: This will only match tuples that have exactly four elements, with the last element being `4`. For example, it matches `(1, 2, 3, 4)` but not `(5, 4)` or `(6, 7, 8, 9, 4)`.

### Example

```rust
fn main() {
    let tup1 = (1, 2, 3, 4);
    let tup2 = (5, 4);
    let tup3 = (6, 7, 8, 9, 4);

    match tup1 {
        (.., 4) => println!("(.., 4) matched"),
        (_, _, _, 4) => println!("(_, _, _, 4) matched"),
        _ => println!("No match"),
    }

    match tup2 {
        (.., 4) => println!("(.., 4) matched"),
        (_, _, _, 4) => println!("(_, _, _, 4) matched"),
        _ => println!("No match"),
    }

    match tup3 {
        (.., 4) => println!("(.., 4) matched"),
        (_, _, _, 4) => println!("(_, _, _, 4) matched"),
        _ => println!("No match"),
    }
}
```

**Output**:
```
(.., 4) matched
(.., 4) matched
(.., 4) matched
```

Here, `(.., 4)` matches all the tuples because they all have `4` as the last element, while `(_, _, _, 4)` only matches `tup1` because it has exactly four elements.

</details>

<details>
<Summary>Day-05: Function in Rust</Summary>


## Main Function
- `main()` function is the entry point of a program.
- Uses snake case as the conventional style for function names.

Example code:
```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Parameters
- A parameter is a special variable that is part of a function’s signature.
- It represents a value that the function expects to receive when it is called.
- Parameters are defined in the function's definition and are used within the function to perform operations.

Example:
```rust
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

## Arguments
- An argument is a concrete value provided to a function when it is called.
- These values are passed to the function’s parameters and used within the function to perform its operations.

Example:
```rust
fn main() {
    another_function(5);
}
```

## Note
- You must declare the type of each parameter. Rust requires type annotations in function definitions. 

## Statements and Expressions
- Function bodies are made up of a series of statements optionally ending in an expression.
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value.

Example:
```rust
{
    let x = 3; // Example of a statement
    x + 1 // Example of an expression
}
```
- Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

## Functions with Return Values
- Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (`->`).
- We can return early from a function by using the `return` keyword and specifying a value.

Example:
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

## Example of Error Related to Return Value

Example:
```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```
- Mismatched types error reveals the core issue with this code. The definition of the function `plus_one` says that it will return an `i32`, but statements don’t evaluate to a value, which is expressed by `()`, the unit type.

</details>