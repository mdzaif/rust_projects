struct Color{
    red: u8,
    green : u8,
    blue : u8
}

pub fn run(){
    let mut blue = Color {red: 0, green: 0, blue: 255};
    print_color(&blue);
    change_color(&mut blue, 12, 45, 255); //pass the mutable reference to let change the value, like ByRef of VB6   
    //show the blue values changed
    print_color(&blue)
}

fn change_color (c: &mut Color, p_red: u8, p_green: u8, p_blue: u8){
    //change the values of c (blue)
    c.red = p_red;
    c.green = p_green;
    c.blue = p_blue;
}

fn print_color (c: &Color){
    println!("Color - R:{}, G: {}, B:{}", c.red, c.green, c.blue)
}