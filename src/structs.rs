// structs -used to create custom data types
//Traditional Struct
struct Color{
    red: u8,
    green:u8,
    blue:u8
 }
 //Tuple Struct
 struct Colors(u8, u8, u8);

 struct Person{
    first_name: String,
    last_name: String

 }
 //functions to use with person Struct
 impl Person{
//construct person
    fn new(first: &str, last: &str)->Person{
        Person { 
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    //get full name

    fn full_name(&self)-> String{
        format!("{} {}", self.first_name, self.last_name)
    }
    //set last name
    fn set_last_name(&mut self, last:&str){
        self.last_name = last.to_string();
    }
    // Name to tuple
    fn to_tuple(self)-> (String, String){
        (self.first_name, self.last_name)
    }
 }
 
pub fn run(){
let mut c = Color {
    red: 225,
    green: 0,
    blue:0
};

c.red=200;

println!("Color: {} {} {}", c.red, c.green, c.blue);

let mut cs =Colors(254, 0,0);
cs.0= 211;
println!("Color: {} {} {}", cs.0, cs.1, cs.2);

let mut p =Person::new("Sara", "Pereira");
println!("Person {}",p.full_name());
p.set_last_name("Gates");
println!("Person {}",p.full_name());
println!("Person Tuple {:?}",p.to_tuple());
// println!("Person {} {}", p.first_name, p.last_name);

}