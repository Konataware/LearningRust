use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Adivinhe o número");

    let secret_number: i8 = rand::thread_rng().gen_range(1..=100);
    println!("O número secreto é {secret_number}");

    println!("Insira seu número");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Muito baixo"),
        Ordering::Greater => println!("Muito alto"),
        Ordering::Equal => println!("GG"),
    }
}
