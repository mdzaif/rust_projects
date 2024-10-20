use rand::Rng;
use std::{cmp::Ordering, vec};

fn get_input() -> String
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
/*fn fibnacci(i: i32) -> i32
{
    if i <= 0
    {
        0
    }else if i == 1{
        1
    }else{
        fibnacci(i-1) + fibnacci(i-2)
    }
}
pub fn run()
{
    let n = get_input().trim().parse::<i32>().unwrap();
        
    for x in 0..n+1{
        print!("{} ", fibnacci(x));
    }
}*/
/*pub fn run(){
    let f = get_input().trim().parse::<f32>().unwrap();

    let c: f32 = ((f-32.0)*5.0)/9.0;

    println!("c = {:.1}", c);
}*/
/*pub fn run()
{
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    
    let guess: i32 = get_input().trim().parse().unwrap();
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}*/
pub fn run()
{
    let mut x = get_input().trim().parse::<i32>().unwrap();
    let mut (bt, wt, tat) = Vec::new();
    for i in bt.iter_mut(){
        *i = get_input().trim().parse::<i32>().unwrap();
    }
    let mut it = 1;
    while(it < x)
    {
        wt
    }
}