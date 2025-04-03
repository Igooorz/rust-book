use std::io;
fn main() {
    println!("Olá, seja bem vindo ao gerador de número Fibonacci!");
    loop {
        println!("Por favor, digite o número de sequência desejado ou \"sair\" para encerrar.");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth).expect("Falha ao ler o comando.");

        if nth.to_uppercase().trim() == "SAIR"{
            break;
        }

        let nth: u8 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, informe um número válido.");
                continue;
            },
        };

        let number = nth_fibonacci(nth);
        println!("O número {} da sequência de fibonacci é o número {}.", nth, number);
    }
}

fn nth_fibonacci(nth: u8) -> u128 {
    match nth {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci(nth-1) + nth_fibonacci(nth-2)
    }
}
