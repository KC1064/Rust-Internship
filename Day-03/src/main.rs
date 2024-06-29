fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    example8();
}

fn example1() {
    // Does not work based on conditionals
    let mut count = 1;
    println!("-----------Example 1-----------");
    loop {
        count += 1;
        println!("Hello, world!");
        if count == 5 {
            break;
        }
    }
}

fn example2(){
    let mut count = 1;
    println!("-----------Example 2-----------");

    loop{
        if(count <= 5){
            println!("Hello, world!");
            count += 1;
        }
        else {
            break;
        }
    }
}

fn example3() {
    // Does not work based on conditionals
    println!("-----------Example 3-----------");
    let mut count = 1;
    let val_to_check = true;
    while val_to_check {
        count += 1;
        println!("Hello, world!");
        if count == 5 {
            break;
        }
    }
}

fn example4() {
    let mut num = 3;
    println!("-----------Example 4-----------");
    while num != 10 {
        println!("{}!", num);
        num += 1;
    }

    println!("Loop ended");
}



fn example5() {
    println!("-----------Example 5-----------");
    for num in 0..10 {
        println!("{}", num);
    }
}

fn example6() {
    let arr = [10, 20, 30, 40, 50];
    println!("-----------Example 6-----------");
    for element in arr.iter() {
        println!("the value is: {}", element);
    }
}


fn example7() {
    println!("-----------Example 7-----------");
    for num in 4..30 {
        if num % 2 == 0 {
            continue;
        } else {
            break;
        }
        println!("{}", num);
    }
}

fn example8() {
    for i in 1..6 {
        for j in 1..i {
            print!("*");
        }
        println!();
    }
}

