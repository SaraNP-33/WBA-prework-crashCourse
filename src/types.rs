/* 
PRIMITIVE TYPES --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128(number of bits they take in memory the larger the number the bigger the bit)
u- unsign - no negative values
Floats: f32, f64
Boolean (bool)
Characters (char)- not a string
Tuples - basically lists
Arrays - they have a fixed length. We use vectors for growable arrays
*/

//Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run(){

    // by default is i32, i don't have to explicitly define that
    let x= 1;

    //by default this is f64
    let y= 2.5;

    //add explicit type
    let z: i64= 45454545454545;

    //Find max size - std - standard library
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active:bool =true;

    //Get boolean from expression
    let is_greater:bool =10<5;

     //Char - unicode it cannot contain more that one Char. \u lets it know it's unicode
     let a1 ='a';
      let face='\u{1F600}';
    println!("{:?}",(x, y, z, is_active, is_greater, a1, face));

}