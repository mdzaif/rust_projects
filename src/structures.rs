//structs -- It create custom data types.

// this is a traditional struct
/*struct Color{
    red: u8,
    green: u8,
    blue: u8,

}*/

//Tuple struct
//struct Color(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person {
    //Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //get full name
    //&self: it is used to reference the struct (here, Person struct).
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name or change last name.
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run()
{
    /*let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };
    //direct access
    c.red = 200;
    println!("Colors: {} {} {}", c.red, c.green, c.blue);*/

    /*let mut c = Color(255, 0, 0);

    //direct assign
    c.0 = 200;

    println!("Colors: {} {} {}", c.0, c.1, c.2);
    */  
    let mut p = Person::new("Mary", "Doe");
    println!("Person: {}", p.full_name());

    p.set_last_name("Williams");
    println!("person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple());
}