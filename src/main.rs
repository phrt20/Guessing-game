extern crate global_counter;

use global_counter::primitive::exact::CounterI16;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let score: u8 = 0;
    static SCORECNT: CounterI16 = CounterI16::new(0);

    println!("Welcome to my super cool ultra fun guessing game. Enter a number from 1-5 and if you are correct, continue on. Good luck!");

    loop {
        println!("
 Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let secretnumber = rand::thread_rng().gen_range(1..=5);

        match guess.cmp(&secretnumber) {
            Ordering::Equal => {
                SCORECNT.inc();
                let score = SCORECNT.get();
                println!("
You got it! You're score is {score}");},
            Ordering::Less => {
                let score = SCORECNT.get();
                println!("
 You finished with a score of {score}!
The secret number was {secretnumber}"
                );
                break;
            }
            Ordering::Greater => {
                let score = SCORECNT.get();
                println!("
 You finished with a score of {score}!
The secret number was {secretnumber}"
                );
                break;
            }
        };
    
    if score == 5 {
        println!("I give up, you win, you are psychic.");
    };
    }
}
