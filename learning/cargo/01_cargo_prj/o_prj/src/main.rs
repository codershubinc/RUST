use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn in_func() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => return println!("You win!"),
        }
    }

}

fn main() {
    in_func();
}
