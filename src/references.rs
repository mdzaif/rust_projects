pub fn run()
{
    /*let mut x = 10;
    //let xr = &x;

    {
        //mutiable reference
    let d = &mut x;

    *d += 1;
    // in here the reference x is in the d and x is modified. but if we want to print the d it gives error.
    let f = &mut x;
    *f += 1;
    }
    
    println!("x is {}",x); */

        let mut x = 5;
    {
        let y = &mut x;

        *y += 1;
    }
    println!("{}", x);
}