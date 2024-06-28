fn main() {
    if_clause();
    if_condition_true();
    if_condition_false();
    equal_condition();
    if_else();
    multiple_if_else();
    if_else_using_variable();
    nested_if_else();
    weather();
}

fn if_clause() {
    if true {
        println!("true");
    }

    if false {
        println!("false");
    }
}

fn if_condition_true(){
    let age = 20;

    if age>18 {
        println!("Adult, Eligible to vote");
    }
}


fn if_condition_false(){
    let age = 15;

    if age>18 {
        println!("Not Adult,not Eligible to vote");
    }
    // NO ELSE TO HANDLE THE EXECEPTION.
}

fn equal_condition(){
    let age = 18;

    if age == 18 {
        println!("Adult, Eligible to vote");
    }
}

fn if_else(){
    let age = 20;

    if age == 18 {
        println!("You are 18");
    }
    else {
        println!("You are not 18");
    }
}

fn multiple_if_else(){
    let age = 35;

    if age<20 {
        println!("Less than 20.");
    }
    else if age<30 {
        println!("Less than 30.");
    }
    else if age<40 {
        println!("Less than 40.");
    }
}

fn if_else_using_variable(){
    let mark = 98;

    let grade = if mark > 90 {"A"} else {"B"};

    println!("Grade: {}",grade);
}

fn nested_if_else(){
    let age = 15;
    if age > 20 {
        if age < 30 {
            println!("you are less than 30");
        } else {
            println!("you are 30 and above");
        }
    } else {
        println!("you are less than 20");
    }
}

fn weather() {
    let is_raining = true;
    let is_windy = true;

    if is_raining && is_windy {
        println!("It's a thunderstorm")
    } else if is_raining || is_windy{
        println!("It's a good weather")

    }else{
        println!("It's a sunny day")
    }
}