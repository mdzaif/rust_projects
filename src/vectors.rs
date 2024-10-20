//vectors -- Resizeable array

use std::mem;

pub fn run()
{
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //Re-assign the value
    println!("{:?}", numbers);
    
    numbers[2] = 20;
    println!("Ressign the values to vector: {:?}", numbers);
    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    println!("Use of push method: {:?}", numbers);

    //Pop off last value
    numbers.pop();

    println!("Use of pop method to pop off last value: {:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get vector length
    println!("Vector length: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //loop through vactor values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Number Vec: {:?}", numbers);
}