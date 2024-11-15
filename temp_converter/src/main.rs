use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut intro = 0;
    
    //flashes welcome message three times
    while intro < 3 {
        print!("WELCOME TO TEMP CONV!");
        io::stdout().flush().unwrap();

        // Wait a second to let the user see the text
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Clear the line
        print!("\r\x1B[2K");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));

        intro += 1;
    }

    loop {
        println!("Enter temperature in fahrenheit.");
        
        let mut fahr_temp = String::new();

        io::stdin()
            .read_line(&mut fahr_temp)
            .expect("Failed to read line");

        let fahr_temp: f64 = match fahr_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };

        let celcius = (fahr_temp - 32.0) * 5.0 / 9.0;

        println!("Your temperature in celcius is {}", celcius);
    }

}

