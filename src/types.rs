/*
Primitive types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128(numbers of bits they take in
memory)
Floats: f32, f64
Boolean: bool
Characters (char)
Tuples
Arrays
*/

/*Rust is statically typed language, which means that it must know the types of all
variables at compile time, however, the compiler can usually infer what type we want to use 
based on the value and how we use it */

// constants = those data types are not chageable. Declare in outside of function and use capital latter.
// IN constant you must defined the data types bit.
const MAXIMUM_NUMBERS: u8 = 24;
use std::i32;
use std::i64;
pub fn run()
{
    //default i32
    let x = 1;

    //default f64;
    let y = 2.5;

    //Add explicit type
    let z: i64 = 4545454545;

    //Find the max size
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);

    //Boolean
    let is_active = true;

    println!("{:?}", (x, y, z, is_active));

    //Get boolean from expression
    let is_greater: bool = 10 > 5;

    println!("{:?}", (x, y, z, is_active, is_greater));

    //character var..
    let a = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a, face));
}