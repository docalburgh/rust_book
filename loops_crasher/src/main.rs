//I want to see how I can crash my terminal or make it go insane through a loop expression.

fn main() {
    let mut counter: i32 = 0;

    loop {
        counter += 1;

        if counter != -1 {
            println!("{counter}");
        }
    }
} //hilarious. Would work even faster if I applied a multiplicative or exponential expression to 'counter.'