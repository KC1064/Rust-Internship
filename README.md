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

<details>
<summary>Day-06: Structs in Rust</summary>

## Structs

Structs are custom data types that let you group/package together different related data and reference them as a single unit.

### Types of Structs

1. **Named-Field Struct**: A struct where all the fields have a name associated with them.
2. **Tuple-Struct**: A struct where fields are defined as a tuple and not named.
3. **Unit-Struct**: A struct where there are no fields defined.

### Creating and Using Structs

You can create an instance of a struct using `let`, as usual, but use a key: value style syntax to set each field. You can access the fields through dot notation, e.g., `origin.x`. Values in structs are immutable by default; use `mut` to make them mutable.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);
}
```

Rust does not support field mutability at the language level.

### Update Syntax

A struct can include `..` to indicate that you want to use a copy of some other struct for some of the values.

```rust
fn main() {
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, ..point };
}
```

### Named-Field Structs

```rust
struct DummyStudent {
    name: String, // Field
    age: u8, // Field
    is_a_student: bool, // Field
}
```

### Tuple Structs

```rust
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Tuple structs are preferred more than simple tuples because they improve readability and it's easier to find the context of the tuple being used.

The members of a tuple struct may be accessed by dot notation or destructuring.

```rust
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let black_r = black.0;
    let Point(_, origin_y, origin_z) = origin;
}
```

### Unit-like Structs

```rust
struct Electron {} // Use empty braces...
struct Proton;     // ...or just a semicolon.

// Use the same notation when creating an instance.
let x = Electron {};
let y = Proton;
let z = Electron; // Error
```

A struct is called 'unit-like' because it resembles the empty tuple, `()`, sometimes called 'unit'.

### Shorthand Representation

If the names of the fields are the same as the names of the variables being assigned to them, then we can use the shorthand notation.

Example:

```rust
struct Student {
    sgpa: f32,
    age: u8, // MAX: 255
    is_a_student: bool,
    sic: u32,
    year: u16,
}

let student_age = 20;
let is_a_student_var = true;
let student_sic = 123456;
let student_year = 2021;

let student = Student {
    sgpa: 9.5,
    age: student_age,
    is_a_student: is_a_student_var,
    sic: student_sic,
    year: student_year,
};
```

### Nested Structure

```rust
struct Library {
    name: String,
    location: String,
    books: Book, // Only one book in the library
}

struct Book {
    author: String,
    book_details: BookDetails,
}

struct BookDetails {
    name: String,
    isbn: String,
}
```

## Implementation in Rust

### impl

An implementation block, denoted by the keyword `impl` followed by the struct's name, is used to define methods and functions for that struct. A struct can have multiple `impl` blocks.

### Methods

Methods are like functions, declared with `fn`, that can have parameters and a return value. They differ from functions as they are defined within a struct and always have `self` as their first parameter.

### self

The `self` keyword represents the instance of the struct the method is called on. It can be used as `self` (shorthand for `self: Self`), `&self` (for `self: &Self`), or `&mut self` (for `self: &mut Self`). This shorthand allows for less repetition, especially with generic types.

### Associated Functions

Functions inside an `impl` block that do not take `self` as a parameter are called associated functions. They are associated with the struct but do not operate on an instance of it. These are often used as constructors to return a new instance of the struct.

### Example

```rust
impl Student {
    fn new(sgpa: f32, age: u8, is_a_student: bool, sic: u32, year: u16) -> Student {
        Student {
            sgpa,
            age,
            is_a_student,
            sic,
            year,
        }
    }

    // printing the student's details
    fn print_student_details(&self) {
        self.sgpa;
        self.sic;
        println!("Student Details: ");
    }

    fn compare_student_sgpa(&self, other: Student) {
        if self.sgpa > other.sgpa {
            println!("Self has a higher SGPA");
        } else {
            println!("Other has a higher SGPA");
        }
    }
}

fn something_outside_impl() {
    let raj: Student = Student::default();
    let jay: Student = Student {
        sgpa: 9.0,
        age: 20,
        is_a_student: true,
        sic: 123456,
        year: 2021,
    };

    let new_student = Student::new(8.5, 20, true, 123456, 2021);
    raj.compare_student_sgpa(jay);
    jay.compare_student_sgpa(raj);
}
```
</details>

<details>

<summary>Day 07: Rust Enums</summary>

## Definition
Enums (or enumerations) are a user-defined data type that allows us to select a value from a list of related values.

### Syntax
```rust
enum Sport {
    Basketball,
    Volleyball,
    Football,
    Cricket,
}
```
Created an enum named `Sport` with a list of values: `Basketball`, `Volleyball`, `Football`, and `Cricket`. These enum values are known as variants.

## Accessing Enum Variants
```rust
enum Direction {
    North,
    East,
    South,
    West,
}
```
### To create instances of enum variants:
```rust
let north = Direction::North;
let east = Direction::East;
let south = Direction::South;
let west = Direction::West;
```

### Example: Enum Datatype
```rust
fn main() {
    // Define enum Direction
    #[derive(Debug)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    // Initialize and access enum variants
    let north = Direction::North;
    let east = Direction::East;
    let south = Direction::South;
    let west = Direction::West;

    // Print enum values
    println!("{:?}", north);
    println!("{:?}", east);
    println!("{:?}", south);
    println!("{:?}", west);
}
```
Note: We have used `#[derive(Debug)]` above the enum definition. It allows Rust to print the variants inside the enum.

## Initializing Enum Variants with Values
```rust
fn main() {
    // Define enum
    #[derive(Debug)]
    enum Result {
        Score(f64),
        Valid(bool),
    }

    // Initialize enum with values
    let num = Result::Score(3.14);
    let bool = Result::Valid(true);
    
    println!("num = {:?}", num);
    println!("bool = {:?}", bool);
}
```

## Enum with Different Data Types
### Struct Variant
Syntax:
```rust
enum Game {
    Quit,
    Position { x: i32, y: i32 },
}
```

### Tuple Variant
Syntax:
```rust
enum Game {
    Quit,
    ChangeBackground(i32, i32, i32),
}
```

### String Variant
Syntax:
```rust
enum Game {
    Quit,
    Print(String),
}
```

### Example
```rust
fn main() {
    // Define enum with multiple variants and data types
    #[derive(Debug)]
    enum Game {
        Quit,
        Print(String),
        Position { x: i32, y: i32 },
        ChangeBackground(i32, i32, i32),
    }

    // Initialize enum with values
    let quit = Game::Quit;
    let print = Game::Print(String::from("Hello World!"));
    let position = Game::Position { x: 10, y: 20 };
    let color = Game::ChangeBackground(200, 255, 255);

    // Print enum values
    println!("quit = {:?}", quit);
    println!("print = {:?}", print);
    println!("position = {:?}", position);
    println!("color = {:?}", color);
}
```

## Comparing Two Instances of Enum
```rust
let rgb_red = ColorModel::RGB(255, 0, 0);
let rgba_red = ColorModel::RGBA(255, 0, 0, 255);
// With named members, use curly braces and the properties' names
let cmyk_black: ColorModel = ColorModel::CMYK{cyan: 0, magenta: 0, yellow: 0, key: 255};

rgb_red == rgba_red; // false
```

## Define Methods on Enums
```rust
impl Message {
    fn call(&self) {
        // Method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## Pattern Matching in Enums
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### Another Example
```rust
#[derive(PartialEq, Clone)]
enum ColorModel {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    CMYK { cyan: u8, magenta: u8, yellow: u8, key: u8 },
}

impl ColorModel {
    pub fn to_hex(&self) -> String {
        match self {
            ColorModel::RGB(red, green, blue) => format!("#{:X}{:X}{:X}", red, green, blue),
            _ => self.to_rgb().to_hex(),
        }
    }

    fn to_rgb(&self) -> Self {
        match self {
            ColorModel::RGB(_, _, _) => self.clone(),
            ColorModel::RGBA(red, green, blue, alpha) => {
                let red: u8 = (1 - alpha) * 255 + alpha * red;
                let green: u8 = (1 - alpha) * 255 + alpha * green;
                let blue: u8 = (1 - alpha) * 255 + alpha * blue;

                ColorModel::RGB(red, green, blue)
            },
            ColorModel::CMYK { cyan, magenta, yellow, key } => {
                let red = 255 * (1 - cyan) * (1 - key);
                let green = 255 * (1 - magenta) * (1 - key);
                let blue = 255 * (1 - yellow) * (1 - key);

                ColorModel::RGB(red, green, blue)
            },
        }
    }
}
```
</details>

<details>
<summary>Day-08: Ownership in Rust</summary>

## Ownership
In Rust, ownership and borrowing are the pillars of memory safety and concurrency.

### Ownership
- Ensures that memory is managed efficiently and safely.
- Rust’s move semantics enable efficient memory management by transferring ownership between variables.

```rust
fn main() {
    let original = String::from("Hello");
    let moved = original; // Ownership transferred to 'moved'

    // Error: 'original' no longer accessible
    println!("{}", original);
}
```

## Stack and Heap
- A stack operates on a Last In, First Out (LIFO) principle, like a stack of plates where you add to the top and remove from the top. Adding is called pushing, and removing is called popping. All data on the stack must have a fixed, known size. Data with unknown or variable sizes must be stored on the heap.
- The heap is less organized, you request space, the allocator finds a spot, marks it as used, and returns a pointer. This is called allocating. The pointer, stored on the stack, leads to the data on the heap. It's like being seated at a restaurant: the host finds a table, and latecomers ask where you're seated to find you.
- Pushing to the stack is faster than heap allocation because the stack's location is always known, while the heap requires searching for available space and bookkeeping.
- Accessing heap data is slower than stack data due to the need to follow pointers. Processors work more efficiently with data stored closely together, like on the stack, rather than scattered, like on the heap.
- When a function is called, its parameters and local variables are pushed onto the stack. Once the function finishes, these values are popped off the stack.
- Ownership addresses the challenges of tracking, minimizing duplicates, and cleaning up unused data on the heap. Understanding ownership clarifies its purpose in managing heap data, reducing the need to think about the stack and heap frequently.

## Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope
```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward
}                      // this scope is now over, and s is no longer valid
```
- When s comes into scope, it is valid.
- It remains valid until it goes out of scope.

## String Type
- The types covered previously are of a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.
- Rust has a second string type, String. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the `from` function.

```rust
let s = String::from("hello");
```

## Mutated String
```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{s}"); // This will print `hello, world!`
```

## Variables and Data Interacting with Move

### For Integers:
```rust
let x = 5;
let y = x;
```
- This binds the value 5 to `x` and then copies it to `y`, resulting in two variables, `x` and `y`, both holding the value 5. Since integers have a fixed size, both values are stored on the stack.

### For Strings:
```rust
let s1 = String::from("hello");
let s2 = s1;
```
- A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
![Strings-1](https://doc.rust-lang.org/book/img/trpl04-01.svg)
- Assigning `s1` to `s2` copies the String's pointer, length, and capacity from the stack, but not the actual data on the heap.
![Strings-1](https://doc.rust-lang.org/book/img/trpl04-02.svg)


## Variables and Data Interacting with Clone
- If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`.
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

## Stack-Only Data: Copy
```rust
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```
- Types with a known size at compile time, like integers, are stored on the stack, making value copies quick and straightforward. Thus, there's no need to prevent `x` from being valid after creating `y`, and calling `clone` isn't necessary as deep and shallow copying are identical in this case.


</details>


<details> 
<summary>Day-09: Borrowing in Rust</summary>

## Mechanism of Borrowing
- Borrowing allows a function or method to temporarily borrow a reference to a value owned by another part of the program.
- It enables a function or method to use a value without taking ownership of it by passing a reference to the value instead of the value itself.

## Types of References
1. **Immutable references**: Allow the borrower to read the value but not modify it.
2. **Mutable references**: Allow the borrower to read and modify the value.

## Examples

### Mutable Borrowing

```rust
fn main() {
    let mut x = 5;
    {
        let y = &mut x; // mutable borrow of x
        *y += 1;
        println!("y: {}", y); // prints "y: 6"
    } // y goes out of scope and the mutable borrow ends
    println!("x: {}", x); // prints "x: 6"
}
```
- We create a mutable variable `x` and a mutable borrow `y`. By dereferencing `y`, we modify `x`. After the inner block, where `y` is valid, `y` goes out of scope, and `x` can be used safely again.

### Immutable Borrowing

Immutable references allow the borrower to read the value but not modify it.

```rust
fn main() {
    let x = 10;
    print_int(&x); // immutable borrow of x
}

fn print_int(v: &i32) {
    println!("{}", v);
}
```
- We create an integer `x` and then pass an immutable borrow of `x` to the `print_int` function. The `print_int` function then prints the value of the integer. Because the borrow is immutable, we cannot modify `x` within the function.

### Mutable Borrowing with Functions

Mutable references allow the borrower to both read and modify the value.

```rust
fn main() {
    let mut x = 10;
    modify_int(&mut x); // mutable borrow of x
    print_int(&x); // immutable borrow of x
}

fn modify_int(v: &mut i32) {
    *v += 5;
}

fn print_int(v: &i32) {
    println!("{}", v);
}
```
- In this example, we create a mutable integer `x`, then pass a mutable borrow of `x` to the `modify_int` function, which modifies its value. Afterward, we pass an immutable borrow of `x` to the `print_int` function, which prints the modified value of `x`.

## Borrowing Rules
- At any given time, you can have either one mutable reference or any number of immutable references to a value.
- References must always be valid, meaning they must point to a valid memory location.
- The borrow checker enforces these rules at compile time, so the program will not compile if it violates them.

## Ownership and Borrowing in Structs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    let area = rect.area(); // immutable borrow of rect
    println!("area: {}", area);
}
```
- We define a struct `Rectangle` that has two fields, `width` and `height`. We then implement a method `area` for the `Rectangle` struct that calculates the area of the rectangle. Finally, we create a `rect` instance of the `Rectangle` struct and pass an immutable borrow of `rect` to the `area` method to calculate the area of the rectangle.

</details>

<details>

<summary>Day-10: Generics</summary>


## Generic Functions

- **Definition**: When I use generic functions, I can operate on different types of data by using placeholders for the data types they work with. These placeholders are called type parameters.

### Example:
```rust
fn print_type<T>(_arg: T) { 
    println!("Type is {}", std::any::type_name::<T>()); 
}
```
- In this example, `print_type` is a function that accepts a single argument `_arg` of type `T`.
- The `<T>` syntax indicates to Rust that `T` is a type parameter.
- I use `std::any::type_name` to print the type's name.

## Generic Structs

- **Definition**: When I use generic structs, I can create data structures that hold values of different types. These structs use type parameters to allow flexibility in the values they store.

### Example:
```rust
struct Circle<T> { 
    diameter: T, 
    height: T,
}

fn main() {
    let object1 = Circle { diameter: 1, height: 2 }; 
    // diameter and height are both i32
    let object2 = Circle { diameter: 3.14, height: 2.71 }; 
    // diameter and height are both f64
    let object3 = Circle { diameter: "hello", height: "world" }; 
    // diameter and height are both &str
}
```
- The `Circle` struct has two fields: `diameter` and `height`, both of type `T`.
- The `<T>` syntax specifies that `T` is a type parameter.
- I can create instances of this struct with any type, as shown in the examples.

## Generic Enums

- **Definition**: When I use generic enums, I can define enums that allow for multiple types with different variants. Each variant can have its own set of associated data of different types.

### Example:
```rust
enum Result<T, E> { 
    Ok(T), 
    Err(E), 
}

fn main() {
    let result1: Result<i32, &str> = Result::Ok(42); 
    // Ok variant with i32
    let result2: Result<i32, &str> = Result::Err("error"); 
    // Err variant with &str
}
```
- The `Result` enum has two variants: `Ok` and `Err`.
- `Ok` takes a value of type `T`, and `Err` takes a value of type `E`.
- The `<T, E>` syntax specifies that both `T` and `E` are type parameters.
- I find this enum useful for representing the result of an operation that may either succeed (`Ok`) or fail (`Err`).

## Key Points

- **Generic Functions**: Allow me to create functions that can operate on any data type using type parameters.
- **Generic Structs**: Enable me to create data structures that store values of various types, providing flexibility and reusability.
- **Generic Enums**: Allow me to define enums with variants that can store different types, making error handling more type-safe.

- Generics are powerful tools in Rust that help me write flexible, reusable code while maintaining type safety. By using type parameters, I can create functions, structs, and enums that work with any data type, reducing code duplication and increasing robustness.

</details>

<details>

<summary>Day-11: Error Handling</summary>


## Types of Errors

### 1. Unrecoverable Errors
- Errors from which a program stops its execution. As the name suggests, we cannot recover from unrecoverable errors.
- These errors are known as panic and can be triggered explicitly by calling the `panic!` macro.

#### Example:
```rust
fn main() {
    println!("Hello, World!");

    // Explicitly exit the program with an unrecoverable error
    panic!("Crash");
}
```

Output:
```
Hello, World!
thread 'main' panicked at 'Crash', src/main.rs:5:5
```

- Unrecoverable errors are also triggered by taking an action that might cause our code to panic.

#### Example:
```rust
fn main() {
    let numbers = [1, 2, 3];

    println!("unknown index value = {}", numbers[3]);
}
```

Output:
```
error: this operation will panic at runtime
 --> src/main.rs:4:42
  |
4 |     println!("unknown index value = {}", numbers[3]);
  |                                          ^^^^^^^^^^ index out of bounds: the length is 3 but the index is 3
  |
```
- Rust stops us from compiling the program because it knows the operation will panic at runtime.

### 2. Recoverable Errors
- Recoverable errors are errors that won't stop a program from executing. Most errors are recoverable, and we can easily take action based on the type of error.

#### Example:
```rust
use std::fs::File;

fn main() {
    let data_result = File::open("data.txt");

    // using match for Result type
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file: {:?}", data_file);
}
```

## Result Enum:
- In the above example, the return type of the `File::open("data.txt")` is a `Result<T, E>`
- The `Result<T, E>` type returns either a value or an error in Rust. It is an enum type with two possible variants.

```
Ok(T) → operation succeeded with value T
Err(E) → operation failed with an error E
```

## Difference Between `Result` and `Option` enum in Rust
- `Option` enum can return `None`, which can indicate failure.
- `Option` is about `Some` or `None` (value or no value)
- `Result` is about `Ok` or `Err` (result or error result)

</details>

<!-- <details>

<summary>Day-12:</summary>

</details>

<details>

<summary>Day-13:</summary>

</details>

<details>

<summary>Day-14:</summary>

</details>

<details>

<summary>Day-10:</summary>

</details> -->