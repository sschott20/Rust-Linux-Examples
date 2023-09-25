use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {number}");

    loop {
        println!("Guess a number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
