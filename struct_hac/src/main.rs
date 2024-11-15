fn main() {
    #[derive(Debug)]
    struct Movie {
        name: String,
        director: String,
        rating: i32,
    }

    let sorcerer = Movie {
        name: String::from("Sorceror"),
        director: String::from("William Friedkin"),
        rating: 83,
    };

    println!("The director of {} is {}", sorcerer.name , sorcerer.director);
}
