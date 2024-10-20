//enum: enums are types which have a few definite values

enum Movement{
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_dot(m: Movement) {
    //Perform action defending on action
    match m{
        Movement::Up => println!("dot moving up"),
        Movement::Down => println!("dot moving down"),
        Movement::Left => println!("dot moving left"),
        Movement::Right => println!("dot moving right")
    }
}

pub fn run()
{
    let dot1 = Movement::Right;
    let dot2 = Movement::Left;
    let dot3 = Movement::Up;
    let dot4 = Movement::Down;

    move_dot(dot1);
    move_dot(dot2);
    move_dot(dot3);
    move_dot(dot4);

}