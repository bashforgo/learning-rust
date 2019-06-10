use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);


    loop {
        println!("Enter the guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read_line");

        let guess: u32 = match guess.trim().parse() {
            Ok(42) => {
                println!("I like your thinking");
                42
            }
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you got it");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
