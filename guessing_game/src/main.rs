use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);

    println!("Guess the number!");

    // while true loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // read line from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare guess to random number
        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
