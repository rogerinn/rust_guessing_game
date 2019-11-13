use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("===Adivinhe o número!===");

    let secret_number = rand::thread_rng().gen_range(1, 101); //Número de 1 a 100
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Insira um número:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Falhou ao ler linha.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Insira um número válido!");
                continue;
            }
        };

        println!("Número inserido: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            }
        }
    }
    
}
