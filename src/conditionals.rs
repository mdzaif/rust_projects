//Conditionals -- Used to check condition of  something and  act on the  result
// find condtion expresion: > < >= <= 
//operator: && || 
pub fn run()
{
    let age = 22;
    let check_id: bool = false;

    //If/Else condional
    /*if age >= 21 && check_id {
        println!("ok");
    }else if age < 21 && check_id {
        println!("Sorry your not ok");
    }else{
        println!("I'll need to check your id");
    }*/

    let is_of_age = if age >= 21 { true } else { false };

    println!("Is of Age: {}", is_of_age);
}