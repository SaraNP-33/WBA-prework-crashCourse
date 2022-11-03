//Variables hold primitive data or references to data
//variables are immutable by default - by default can't reasigned them
//rust is a block-scoped language -if variable is assigned to fn it pertains to that scope

pub fn run(){
let name = "Sara";

//in order to reassinged a variable we need to add mut to the variable
let mut age=35;
//duplicate this println so it reads both variables - the original and the reassinged
println!("My name is {} and I am {}", name, age);
age=36;
println!("My name is {} and I am {}", name, age);
}