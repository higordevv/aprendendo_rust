// Indicar biblioteca externa
extern crate rand;

// use rand::Rng;
use rand::{thread_rng, Rng};
use std::io;
fn main() {
    println!("Adivinhe o número!\n");

    let numero_secreto = thread_rng().gen_range(1, 101);

    println!("O numero fica entre 1 e 100");

    println!("Digite o seu palpite.");

    let mut palpite = String::new();
    io::stdin()
        .read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    // {} é p curinga
    println!("Você disse: {palpite}");
}
