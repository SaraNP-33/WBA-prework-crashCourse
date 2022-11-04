// functions -used to store blocks of code for re-use

pub fn run(){
greeting("Olá", "Jeff");

// Bind function values to variables
let get_sum= add(5,5);
println!(" Sum: {}", get_sum);

//closure
//can use outside variables 
let n3:i32 =10;
let add_nums= | n1: i32, n2: i32| n1+n2+n3;
println!("C Sum: {}", add_nums(3,3) );
}

fn greeting( greet: &str, name: &str){
    println!("{} {}, nice to meet you", greet, name)
}

//-> is to return
fn add(n1: i32, n2:i32) -> i32{
    //we don't use a semicolon to show what we want to return
    n1 + n2
}