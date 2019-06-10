use std::io;

fn main() {
    println!("Guess the number");
    println!("Enter the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read_line");

    println!("Your guess: {}", guess);
}
