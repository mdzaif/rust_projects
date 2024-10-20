//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language.
pub fn run2()
{
    let name = "Md. Zaif Imam Mahi";
    let mut age = 20;
    println!("My name is {} and I am {} years old.", name, age);
    age = 21;
    println!("My name is {} and I am {} years old.", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple variables
    let ( my_name, my_age ) = ( "Zaif", 21);
    println!("{} is {}", my_name, my_age);
}