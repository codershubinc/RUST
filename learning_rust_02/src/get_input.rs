use std::io;

pub fn input_num() -> u128 {
    let mut input = String::new();

    println!("Enter a number to calculate the factorial: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: u128 = input.trim().parse().expect("Please type a number!");
    return number;
}

pub fn input_string() -> String {
    let mut input = String::new();

    println!("Enter a string: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}
