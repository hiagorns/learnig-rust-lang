fn main() {
    let number = 5;

    if number > 5 {
        println!("number is greater than 5");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is equal to 5");
    }

    
    let condition = false;
    let x = if condition {5} else {6};

    println!("The value of x is: {x}");

    let a;
    let cond = false;

    if cond {
        a = 1;
    } else {
        a = 2;
    }

    println!("The value of a is: {a}");

// Loops

    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

// Loop: com label

    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");

// while

    let mut dec_count = 5;

    while dec_count > 0 {
        println!("dec_count: {dec_count}");

        dec_count -= 1;

    }

// for
    
    let a_ray = [10, 20, 30, 40, 50];

    for element in a_ray {
        println!("The value is: {element}");
    }

    for index in (1..4).rev() {
        println!("On index {index}: {}", a_ray[index]);
    }
}