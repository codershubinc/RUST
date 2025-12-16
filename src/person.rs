pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
    pub fn say_hello(&self) {
        println!(
            "Hello, my name is  {} ,  and age is {}",
            self.name, self.age
        )
    }
}
