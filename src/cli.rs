use std::env;

pub fn run(){
    let args: Vec<String>=env::args().collect();
    let command =args[1].clone();
    let name ="Jen";
    let status ="100%";

    // println!("Args: {:?}", args);
    // println!("Command: {:?}", command);

    if command == "hello"{
        //cargo run hello
        println!("Hi {}, how are you?", name);
    }else if command == "status" {
        //cargo run status
        println!("Status is {}", status );
    }else {
        //cargo run hi (or anything else)
        println!("That is not a valid command")
    }
}