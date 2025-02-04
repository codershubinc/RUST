mod factorial;
mod get_input;
mod printing_pattern;

fn main() {
    let number = get_input::input_num(
        "Enter a number to calculate the factorial and print a pattern".to_string(),
    );
    let result = factorial::factorial(number);
    println!("The factorial of {} is {}", number, result);
    printing_pattern::triangle_pattern(number);
}
