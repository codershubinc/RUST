pub fn factorial(num: i64) -> u64 {
    let mut fact: u64 = 1;

    for i in 1..num + 1 {
        fact *= i as u64;
    }
    fact
}
