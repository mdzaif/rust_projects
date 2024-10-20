// function -- to store the code for re-use

// when we return any values from function we did not give the ; to the 

// syntax(return types): fn func_name(var_name: bit_size) -> data_type_for_return {
    //funtion body
//}


pub fn run()
{
    /*greeting("Hello", "Zaif");

    //Bind function values to variable
    let get_sum = add(5, 5);
    println!("The sum is: {}", get_sum);

    //Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;

    println!("The sum is: {}", add_nums(3, 3));*/

    let even_num: bool = is_even(20);
    println!("Result even or odd: {}", even_num);
}

/*fn greeting(greet: &str, name: &str)
{
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32
{
    n1 + n2
}*/

fn is_even(num: u32) -> bool{
    //num % 2 == 0
    
    // also we can return and it considered as old version.
    return num % 2 == 0;
}

