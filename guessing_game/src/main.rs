use std::io;

fn main() {
    println!("Guess the number")

    println!("Please input your guess:")

    let mut /* modifiable */ guess = String::new();
    io::stdin()
        .read_line(&mut guess) // insert to mutable var.
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}