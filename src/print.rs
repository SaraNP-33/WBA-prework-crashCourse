pub fn run(){
    // print to console
    println!("Hello from the print.rs file");

    // BASIC FORMATTING

    //When printing something we always need a placeholder, can't just plug in interger
    println!("Number: {}", 1);

    //Multiple placeholders
    println!("{} is from {}", "Sara", "the Azores");

    //POSITIONAL ARGUMENTS

    //we use the indexs to indicate the position of the arguments
    println!("{0} is from {1} and {0} likes to {2}", "Sara", "the Azores", "code");

    //NAMED ARGUMENTS
    println!("{name} likes to play {activity}", name="John", activity="Baseball");

    //PLACEHOLDER TRAITS
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    //PLACEHOLDER FOR DEBUG TRAIT
    //comes in handy when you want to print out for example an entire array

    println!("{:?}", (12, true, "hello"));

    //BASIC MATH
    
    println!("10 + 10 ={}", 10 + 10);
}