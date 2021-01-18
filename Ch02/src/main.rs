use std::io;

fn main() {
    println!("Guess the number! ~ TeamIF - RustStudy");
    println!("Please input your guess :D");
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line :(");
    println!("You guessed {}", guess);
}