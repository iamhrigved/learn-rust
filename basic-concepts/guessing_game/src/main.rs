use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); // we have to declare this variable again and again because
                                       // rust concats the string when the old string is not
                                       // cleared, so every time the string needs to be empty.

        io::stdin()
            .read_line(&mut guess)
            .expect("Opps! An ERROR occured");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Right guess! You win.");
                break;
            }
        }
    }
}
