//Primitive str =immutable fixed length string somewhere in memory
// String= growable, heap-allocated data structure -use when you need to mofify or own string data

pub fn run(){
let hello ="Hello"; //primitive str
let mut hello2= String::from("Hello "); //growable string
println!("{}", hello2);
//get length 
println!("Length: {}", hello2.len());

hello2.push('W'); //This only works with the hello2 after we add mut and also because it's a growable string
hello2.push_str("orld!");// same as above

//capacity in bytes
println!("Capacity: {}", hello2.capacity());

//check if empty
println!("Is Empty: {}", hello2.is_empty());

//contains a substring?
println!("Contains 'World' {}", hello2.contains("World"));

//replace
println!("Replace : {}", hello2.replace("World", "Universe"));

//Loop through string by whitespace

for word in hello2.split_whitespace(){
    println!("{}", word);
}

//create string with capacity
let mut s =String::with_capacity(10);
s.push('a');
s.push('b');

//assertion testing
assert_eq!(2, s.len());
assert_eq!(11, s.capacity()); // if it's 10 nothing happens because it's true. with 11 the assertion fails

println!("{}", s);

}