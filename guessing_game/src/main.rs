
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let sn = rand::thread_rng().gen_range(1..=100);
    println!("The number is: {sn}");
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>continue,
        };


        println!("you guessed : {guess}");

        match guess.cmp(&sn) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Correct Guess");   
                break;
            }
        }
    }
}
