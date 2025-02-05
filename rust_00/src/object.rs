pub fn obj() {
    println!("object");
    
    let person = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };
    
    println!("name of the person is => {} {}", person.first_name, person.last_name);
    
}

struct Person {
    first_name: String,
    last_name: String,
}