pub fn run()
{
    //let mut count = 0;
    //infinte loop
    /*loop{
        count += 1;
        println!("Number: {}", count);

        if count == 20{
            break;
        }
    } */

    //While loop(FizzBuzz)
    /*while count <= 100{
    if count % 15 == 0
    {
        println!("fizzbuzz");
    }else if count % 3 == 0
    {
        println!("fizz");
    }else if count % 5 == 0
    {
        println!("buzz");
    }else
    {
        println!("{}", count);
    }
    // Inc
    count += 1;
    }*/

    // For range
    /*for x in 0..100 {
        if x % 15 == 0
    {
        println!("fizzbuzz");
    }else if x % 3 == 0
    {
        println!("fizz");
    }else if x % 5 == 0
    {
        println!("buzz");
    }else
    {
        println!("{}", x);
    }
    
    }*/

    /*let numbers = 1..10;

    for i in numbers{
        println!("{}", i);
    }*/

    let animals = vec!["Rabbit", "Dog", "Cat"];
    
    /* //the normal for loop for vector iteration.
    for a in animals.iter(){
        println!("The animal name is {}", a);
    }*/

    for (index, a) in animals.iter().enumerate() {
        println!("The index is {} and the animal is {}", index, a);
    }
}