use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número de 1 a 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Por favor, digite seu chute.");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler o chute.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Você chutou: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Seu chute foi menor..."),
            Ordering::Equal => {
                println!("Boa! Você Acertou!!!");
                break;
            },
            Ordering::Greater => println!("Seu chute foi maior..."),
        }
    }
}
