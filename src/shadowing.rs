pub fn run()
{
    let mut x = 10;
    {
        x = 4; // compiler count this line and the value of x is 4;
        // if we write let x = 5; then the code didn't count this line and get error message.
        //but print the x = 10
    }
    println!("X is {}", x);

    //change the data types in single variable
    let x = "X is a string";
    println!("{}", x);

    let x = true;
    println!("{}", x);
}