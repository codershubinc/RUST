pub fn triangle_pattern(a: u128) -> u128 {
    for i in 1..=a {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
    0
}
