use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Adivinhe o número secreto.");
    
    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("ERRO: NÃO PUDE LER A LINHA.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // CATCH ALL xpr
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => println!("GG"),
        }
    }
}
