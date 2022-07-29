
use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    let mut score: u8 = 0;

    println!("Welcome to my super cool ultra fun guessing game.");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut secretnumber = rand::thread_rng().gen_range(1..=5);

        match guess.cmp(&secretnumber) {
            Ordering::Equal => println!("You got it!"),
            Ordering::Less => {
                println!("You finished with a score of {score}!
                The secret number was {secretnumber}");
                break;
            }
            Ordering::Greater => {
                println!("You finished with a score of {score}!
                The secret number was {secretnumber}");
                break;
            }
        };
        let score = score + 1;
    }
}
