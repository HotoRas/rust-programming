use std::io;
use std::cmp::Ordering;

// cargo
use rand::Rng;
// end cargo

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100); // range exp: start..=end
    //println!("Secret: {secret_number}")

    println!("Please input your guess:");

    let mut /* modifiable */ guess = String::new();
    io::stdin()
        .read_line(&mut guess) // insert to mutable var.
        .expect("Failed to read line");
    let guess: u32 /*uint32*/ = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!"),
    }
}