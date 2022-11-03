//Arrays -Fixed list where elements are the same data types

use std::mem;

pub fn run(){
//the datatype and the lenght have to be exact
//if I remove the 5 I will get an error
//if I add a string i.e "hello", i will get an error
 let mut numbers:[i32; 5]=[1,2,3,4,5];

 //re-assign value
 numbers[2] = 20;

 println!("{:?}", numbers);

 //get single val
 println!("Single Value:{}", numbers[0]);

 //get array length
 println!("Array Length: {}", numbers.len());

 //Arrays are stack allocated
 println!("Array occupies {} bytes", mem::size_of_val(&numbers));

 //Get Slice

 let slice: &[i32]=&numbers[1..3];

 println!(" Slice: {:?}", slice);

}