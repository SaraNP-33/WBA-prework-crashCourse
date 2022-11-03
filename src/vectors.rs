//Vectors - resizable arrays

use std::mem;

pub fn run(){

 let mut numbers:Vec<i32>=vec![1,2,3,4,5];

 //re-assign value
 numbers[2] = 20;

 //add on to vector
 numbers.push(6);
 numbers.push(7);

 //pop off last value
 numbers.pop();

 println!("{:?}", numbers);


 //get single val
 println!("Single Value:{}", numbers[0]);

 //get vector length
 println!("Vector Length: {}", numbers.len());

 //Arrays are stack allocated
 println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

 //Get Slice

 let slice: &[i32]=&numbers[1..3];

 println!(" Slice: {:?}", slice);

 //Loop through vector values

 for x in numbers.iter(){
    println!("Number: {}", x)
 }

 //Loop & mutate values
 for x in numbers.iter_mut(){
    *x *=2
 }
println!("Numbers Vec: {:?}", numbers);
}