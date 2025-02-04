mod factorial;
mod get_input;
mod printing_pattern;

fn main() {
    let number = get_input::input_num();
    let result = factorial::factorial(number);
    println!("The factorial of {} is {}", number, result);
    printing_pattern::triangle_pattern(number);
}
