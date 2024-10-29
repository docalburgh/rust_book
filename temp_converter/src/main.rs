//prog converts f to c using input from user.

use std::io;

fn main() {
    println!("Enter temperature in fahrenheit.");
    
    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let fahrenheit: f64 input.trim().parse().expect("Please enter a valid number");
    

}
