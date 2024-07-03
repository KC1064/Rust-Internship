fn main() {
    normal_function();
    another_normal_function();
    fn_with_parameters(12, 36);
    let a = add_ten_to_twenty();
    println!("a: {}", a);
    let b = add(30, 40);
    println!("b: {}", b);
    println!("{}",return_with_if_else());
    let nothing = something();
    println!("Nothing: {}",nothing);
}

fn normal_function() -> () { // Unit type return
    println!("---------------Example 1---------------");
    println!("Normal Function with return type mentioned.");
}

fn another_normal_function() {
    println!("---------------Example 2---------------");
    println!("Normal Function with return type not mentioned.");
}

fn fn_with_parameters(a: i32, b: i32) {
    println!("---------------Example 3---------------");
    println!("Function with Parameters");
    println!("a: {}", a);
    println!("b: {}", b);
}

fn add_ten_to_twenty() -> i32 { // Function that returns a value
    println!("---------------Example 4---------------");
    10 + 20   
}

fn add(a: i32, b: i32) -> i32{
    println!("---------------Example 5---------------");
    // let c = a + b;
    // c
    // return c;  // option 1
    // a + b // option 2
    return a + b; // option 3
}

fn return_with_if_else() -> i32 {
    println!("---------------Example 6---------------");
    let a = 10;
    if a % 2 == 0 {
        return 10;
    } else if a == 20 {
        return 0;
    } else {
        return 1;
    }
}

fn something() -> i32{
    println!("---------------Example 7---------------");
    let a = if 100 > 200 {
        100
    } else {
        200
    };
    return a;
}


// // Arguments - Concrete values of the parameters that are passed to a function.
// // Parameters - Variables in the signature of the function that tell you the type they expect and to which you can pass the concrete values.