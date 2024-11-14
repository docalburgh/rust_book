use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut intro = 0;
    
    while intro < 3 {
        print!("WELCOME TO TEMP CONV!");
        io::stdout().flush().unwrap();

        // Wait a moment to let the user see the text
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Clear the line
        print!("\r\x1B[2K");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));

        intro += 1;
    }

    println!("Enter temperature in fahrenheit.");
    
    let mut fahr_temp = String::new();

    io::stdin()
        .read_line(&mut fahr_temp)
        .expect("Failed to read line");

    let fahr_temp: i32 = match fahr_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
    
}

