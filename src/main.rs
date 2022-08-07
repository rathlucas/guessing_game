use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess what's the number!");

    loop {
        println!("Input your guess: ");

        let random_number = rand::thread_rng().gen_range(1..=100);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The random number is: {random_number}");
                break;
            }
        }

        println!("You guessed: {guess}");
    }
}
