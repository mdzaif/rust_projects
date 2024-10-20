pub fn run3()
{
    //print to console
    println!("Md. Zaif Imam Mahi");
    
    //basic formatting.
    println!("{} are from {}!", "We", "Bangladesh");

    //Positional Arguments.
    println!("{0} are from {1} and {2} likes to {3}!", "We", "Bangladesh", "we", "code");

    //Name Arguments
    println!("{que} are you?", que = "Who");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}