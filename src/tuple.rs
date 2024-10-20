//Tuples group together values of different types
//Max 12 elements

// tuples struct
// syntax: struct tuple_struct_name(parameter_of_data_types);
// example: struct Color(u8, u8, u8);

pub fn run()
{
    // syntax: let mut struct_tuple = tuple_struct_name(data);
    // example: let mut st = Color(255, 0, 0);
    let person: (&str, &str, i8) = ("Zaif", "Bangladesh", 21);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    println!("debug: {:?}", person);

    let numbers = (10, 23, 2.3, false, (1, 3, 5));
    println!("{}", (numbers.4).2);

    let tup1 = (1, 3.3, "Bangladesh");
    let (a, b, c) = tup1;

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}