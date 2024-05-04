use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Using = to be inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Experimentation, woo!
        // Spoiler: it prints nothing. No error though, so that's neat.
        // println!("What happens if you print a newly initialized string? | {guess}");

        // Try to read a line, if error then crash & print simple message.
        // Also, read_line returns a Result. More on Result in ch. 6.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Example of shadowing: another guess is initialised with a different type.
        // parse() works easily here because the var type is specified in let.
        // First example of match statements to work with the Result type!
        let guess: u32 = match guess.trim().parse() {
            // If all is well, parse returns something. Use the something.
            Ok(num) => num,
            Err(_) => {
                println!("Excuse me? What is this? No.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => 
                println!("Too low!"),
            Ordering::Greater =>
                println!("Too high!"),
            Ordering::Equal => {
                println!("Exactly right!");
                break;
            }
        };
    }
}
