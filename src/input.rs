use std::io::stdin;
use std::io::BufRead;
fn get_input() -> String {
    let mut input = String::new();
    stdin()// the rough equivalent of `std::cin`
    .read_line(&mut input)// actually read the line in string.
    .expect("Failed to take input!");// excpection of input. 
    input
}
/*  
    .trim() // ignore whitespace around input
    .parse() // convert to integers
    .expect("Input not an integer");
    println!("You Entered: ");
    //std::io:stdin().read_line(&mut line).unwrap();
    //std::io::input.parse::<i32>().unwrap(); */
pub fn run()
{
    
    //create a get_input() function which pass a string.
    let st = String::new();
    st = get_input();
    let n = get_input().trim().paarse::<i32>().expect("Data can not be read!");
    
    let mut x: i32 = std::io::stdin().lock().lines().next().expect("stdin should be available")
    .expect("Couldn't read from stdin")
    .trim()
    .parse()
    .expect("input was not an integer");
    
}