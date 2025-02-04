pub fn factorial(a: u128) -> u128 {
    if a > 34 {
        println!("The factorial of {} is too large to calculate", a);
        return 1;
    }
    let mut result = 1;
    if a == 1 {
        println!("Factorial is not defined for negative numbers");
        return 1;
    }
    for i in 1..=a {
        result *= i;
        print!("{}", i);
    }
    result
}
