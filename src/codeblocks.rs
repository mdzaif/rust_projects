pub fn run()
{
    let x = 20;
    
    {
        // we can say it is isolated block.
        let y = 5;
        println!("x: {}, y: {}", x, y);//this code will have no error.
    }
    //println!("x: {}, y: {}", x, y);//this have error can not find the y variable.
}