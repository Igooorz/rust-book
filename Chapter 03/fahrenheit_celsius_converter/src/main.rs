use std::io;
fn main() {
    println!("Olá, seja bem vindo ao conversor!");
    'main_loop: loop {
        println!("Se você deseja converter um valor de Fahrenheit para Celsius, digite 1.");
        println!("Se você deseja converter um valor de Celsius para Fahrenheit, digite 2.");
        println!("Para sair, digite 3.");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Falha ao ler o comando.");
        let command: u8 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, informe um número válido.");
                continue 'main_loop;
            },
        };

        let original_unity: &str;
        let new_unity: &str;
        let converter: fn(f32) -> f32;

        match command {
            1 => {
                original_unity = "Fahrenheit";
                new_unity = "Celsius";
                converter = fahrenheit_to_celsius;
            },
            2 => {
                original_unity = "Celsius";
                new_unity = "Fahrenheit";
                converter = celsius_to_fahrenheit;
            },
            3 => {
                println!("Até a próxima!");
                break 'main_loop;
            },
            _ => {
                println!("Por favor, informe um número válido.");
                continue 'main_loop;
            },
        }

        loop {
            println!("Digite a temperatura em graus {original_unity}:");
            let mut temperature = String::new();
            io::stdin().read_line(&mut temperature).expect("Falha ao ler o comando.");
            let temperature: f32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Por favor, informe um número válido.");
                    continue 'main_loop;
                },
            };
            let result = converter(temperature);
            println!("A temperatura convertida é de aproximadamente {result} graus {new_unity}.\n\n");
            break;
        }
    }
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    ((temp - 32.0) * 5.0/9.0).round()
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    ((temp * 9.0/5.0) + 32.0).round()
}
