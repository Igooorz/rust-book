use std::io;
fn main() {
    println!("Olá, seja bem vindo ao conversor!");
    loop {
        println!("Se você deseja converter um valor de Fahrenheit para Celsius, digite 1.");
        println!("Se você deseja converter um valor de Celsius para Fahrenheit, digite 2.");
        println!("Para sair, digite 3.");
        let mut comando = String::new();
        io::stdin().read_line(&mut comando).expect("Falha ao ler o comando.");
        let comando: u8 = match comando.trim().parse() {
            Ok(num) => {if num <= 3 {num} else {
                println!("Por favor, informe um número válido.");
                continue;
            }},
            Err(_) => {
                println!("Por favor, informe um número válido.");
                continue;
            },
        };
        if comando == 3 {
            println!("Até a próxima!");
            break;
        }
        println!("O comando dado foi {comando}")
    }
}
