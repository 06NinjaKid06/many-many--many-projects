use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process;

fn main() {

    println!("Welcome to the guessing game!");
    println!("Type 'quit' to quit");

    loop {

        println!("Guess how many are in the jar! (1-999)");
        println!("   /‾‾‾＼    ");
        println!("  /ooooo＼   ");
        println!(" |ooooooo|   ");
        println!(" |ooooooo|   ");
        println!(" |ooooooo|   ");
        println!(" |_______|   ");

        let secret_number = rand::thread_rng().gen_range(1..=999);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess == "quit" {
            process::exit(0);
        }

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("It was: {secret_number}");

    }

}
