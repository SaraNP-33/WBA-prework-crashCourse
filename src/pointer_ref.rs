// reference pointers -point to a resource in memory
// if we have a primitive value or datatype we can create a variable and point at another value
pub fn run(){
// Primitive Array
let arr1=[1,2,3];
let arr2 =arr1;
println!("Values:{:?}", (arr1, arr2));

//With non- primitives, if you assign another varible to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource
//Vector
let vec1= vec![1,2,3];
let vec2=&vec1;

println!("Values:{:?}", (&vec1, vec2));
}