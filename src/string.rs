//Primitive str = Immutable fixed-length string somewhere in memeory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run()
{
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    //get length
    println!("Length: {}", hello.len());

    //check empty
    println!("Is my string is empty? {}", hello.is_empty());

    hello.push('W'); //push only a character

    println!("{}", hello);

    hello.push_str("orld!");// push a string.

    println!("{}", hello);

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Empty chcek
    println!("Is Empty: {}", hello.is_empty());

    //Contains
    println!("Contain 'World': {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "there"));

    //Loop through string by whitespace
    for word in hello.split_whitespace()
    {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    let mut s = String::new();

    println!("Capacity: {}", s.capacity());

    for _ in 0..5 {
        s.push_str("hello");
        println!("Capacity: {}", s.capacity());
    }
    println!("The string is {}", s);
}