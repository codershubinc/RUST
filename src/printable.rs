pub trait Printable {
    fn format(&self) -> String;
}

// This function accepts ANYTHING that implements Printable
pub fn print_details(item: &impl Printable) {
    println!("Details: {}", item.format());
}
