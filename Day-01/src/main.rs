const THREE_MINUTES: u32 = 3*60;

fn main() {
    println!("Hello, world!");
    immutable_variable_example();
    mutable_variable_example();
    const_example();
    shadowing_example();
    scope_example();
    destructuring_example();
    adv_destructuring_example();
    vary_adv_destructuring_example();
}

fn immutable_variable_example() {
    let x = 5;
    println!("The value of x is: {}", x);

    // x=100;
    // Will not work because immutable variable so value can not be changed
}

fn mutable_variable_example(){
    let mut y = 100;
    println!("The value of x is: {}", y);
    y=999;
    println!("The value of x is: {}", y);
}

fn const_example(){
    println!("--Constant Example--");
    println!("Three minutes in second: {}",THREE_MINUTES);
}

fn shadowing_example(){
    println!("--Shadowing Example--");
    
    let x = 100;
    println!("Value of x: {}",x);

    let x = false;
    println!("Value of x: {}",x);

    let x= 80.43;
    println!("Value of x: {}",x);

    let x = "Hello World";
    println!("Value of x: {}",x);

    let mut y = 2000;  // mut is unnecessary in shadowing
    println!("Value of y: {}",y);

    let mut y = true;
    println!("Value of y: {}",y);
}

fn scope_example(){
    let x = 100;
    {
        let x = 200;
        println!("Value of x: {} Local Scope",x);
    }
    println!("Value of x: {} Global Scope",x);
}

fn destructuring_example(){
    println!("--Destructuring Example--");
    let (a,b,c,d,e) = (10,20,30,40,50);
    println!("Value of a: {}",a);
    println!("Value of b: {}",b);
    println!("Value of c: {}",c);
    println!("Value of d: {}",d);
    println!("Value of e: {}",e);
}

fn adv_destructuring_example(){
    println!("--ADVANCE DESTRUCTURING EXAMPLE--");
    let (first , ..) = (1,2,3,4,5,6,7,8,9);
    println!("Value of first: {}",first);
    let (.. , last) = (1,2,3,4,5,6,7,8,9);
    println!("Value of last: {}",last);

    let my_num = (100,200,300,400,500);
    let (first, ..) = my_num;
    let p = my_num.2;
    println!("Value of p: {}", p);
}

fn vary_adv_destructuring_example(){
    println!("--VARY ADVANCE DESTRUCTURING EXAMPLE--");

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point{x: 10, y: 20};
    let Point{x, y} = p;

    println!("value of x: {}",x);
    println!("value of y: {}",y);
}