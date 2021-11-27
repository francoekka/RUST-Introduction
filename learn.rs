use std::io;
mod print;

fn main(){
    println!("\t\t\t---------------------\n
             \t\t **Welcome to Rust** \n
                \t---------------------\n");

    println!("Enter your Choice which you want to learn :\n\n1. Printing Methods\n");
    //let choice = 1;
    let mut input_string = String::new();
    io::stdin()
    .read_line(&mut input_string)
    .expect("cannot read from stdin");
    let choice:i32 = input_string.trim().parse().expect("Not a valid number");

    
    match choice{
        1 => print::run(),
        _ => println!("does not match")
    }
    
}