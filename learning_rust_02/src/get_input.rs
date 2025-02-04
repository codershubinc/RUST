use std::io;

pub fn input_num(msg: String) -> u128 {
    let mut input = String::new();

    println!("{}", msg);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: u128 = input.trim().parse().expect("Please type a number!");
    return number;
}

pub fn input_string(msg: String) -> String {
    let mut input = String::new();

    println!("{}", msg);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}
