// Constantes // 
const TWO:i32 = 1 + 1;

fn main() {

    println!("{TWO}");

// Mutabilidade //

    // let x = 5; // Sem mut, a variável se torna imutável
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


// Shadowing //
    let y = 5;

    let y = y + 1; // Sombreando

    {
        let y = y * 2; // Sombreando novamente, mas dentro de um novo escopo
        println!("The value of y is: {y}");
    }

    println!("The value of y is: {y}");

}
