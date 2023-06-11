fn five() -> u32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    another_function(5, 'h');

    let y = {
        let x = 5;
        x + 1 // Não deve ter ; para que seu valor seja retornado
    };

    println!("The value of y is: {y}");

    let returned_value = five();
    println!("Return value: {}", returned_value);

    let a = plus_one({
        let y = 1;
        y + 1
    });

    println!("The value of a is: {a}");
}

fn another_function(x: i32, c: char) {
    println!("The value of x is: {x}");
    println!("The value of c is: {c}");
}