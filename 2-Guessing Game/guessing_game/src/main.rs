use std::io; // Biblioteca de input/output que vem com a biblioteca padrão
use rand::Rng; // Adição da trait Rng ao escopo
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //Usa a crate externa rand, e gera um número aleatório no intervalo (1,100]

    // println!("The secret number is: {secret_number}");
   
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // Definição de variável mutável a que é atribuída a função String::new (da biblioteca padrão) que retorna uma nova instância de string
                                       // String é um tipo fornecido pela variável padrão que é 'growablegrowable, UTF-8 encoded bit of text'.
    
        io::stdin() // funcionalidade de entrada
            .read_line(&mut guess) // coloca o que o usuário informar dentro de guess
            .expect("Failed to read line"); // read_line retorna Result (enumaration). Se for variante Ok, retorna o valor dentro. Senão, causa crash
    
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // String para número
        //                                                                        // trim() método de String elimina espaços em branco no inicio e no fim
        //                                                                        // parse() converte de um tipo para outro. Retorna Result
        //                                                                        // exepect() trata as variantes retornadas por parse()
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) { // Expressão de compração de padrões - cmp copara e retorna Ordering, enumeration
            Ordering::Less => println!("Too small!"), // braços de comparação e tratamento
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
