use std::io;
mod factorial;

fn main() {
    let mut lp_cal = 0;
    loop {
        lp_cal += 1;
        let mut input = String::new();

        println!("Enter a number to calculate the factorial: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: u128 = input.trim().parse().expect("Please type a number!");

        if number > 34 {
            println!("The factorial of {} is too large to calculate", number);
            continue;
        }
        let result = factorial::factorial(number);

        println!("The factorial of {} is {}", number, result);

        if lp_cal == 5 {
            break;
        }
    }
}
