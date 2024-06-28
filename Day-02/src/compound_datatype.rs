fn main(){
    tuple_example();
    array_example();

}

fn tuple_example(){
    println!("Tuple Example:");

    let person=('A',21,8.9);
    println!("Value of person is: {:?}",person); //Priniting the whole tuple.

    //Access elements by indices
    let name = person.0;
    let age = person.1;
    let gpa = person.2;

    println!("Name: {}",name);
    println!("Age: {}",age);
    println!("GPA: {}",gpa);

    // Using Destructuring
    let (name,age,gpa) = person;
    println!("Name: {}",name);
    println!("Age: {}",age);
    println!("GPA: {}",gpa);

      // Mutable Tuple
      let mut mutable_tuple = (1, 2, 3);
      println!("The value of mutable_tuple is: {:?}", mutable_tuple);
  
      mutable_tuple.0 = 445159;
      println!("The mutated value of mutable_tuple is: {:?}", mutable_tuple);

}

fn array_example() {
    println!("Array Example");

    let arr_of_numbers = [1, 2, 3, 4, 5];
    println!("The value of arr_of_numbers is: {:?}", arr_of_numbers);

    // Access Elements
    let first_element = arr_of_numbers[0];
    let second_element = arr_of_numbers[1];
    let third_element = arr_of_numbers[2];

    println!("The first element of arr_of_numbers is: {}", first_element);
    println!("The second element of arr_of_numbers is: {}", second_element);
    println!("The third element of arr_of_numbers is: {}\n", third_element);


    let arr_of_repeated_elements = [1; 5];
    let another_arr_of_repeated_elements: [i8; 15] = [2; 15];
    let another_another_arr_of_repeated_elements = [0u8; 15];

    println!("The value of arr_of_repeated_elements is: {:?}", arr_of_repeated_elements);
    println!("The value of another_arr_of_repeated_elements is: {:?}\n", another_arr_of_repeated_elements);
    println!("The value of another_another_arr_of_repeated_elements is: {:?}\n", another_another_arr_of_repeated_elements);
}