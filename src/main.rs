mod factorial;
mod is_even;
mod person;
mod print_length;
mod shapes;

fn main() {
    println!("is 10 odd {}", !is_even::is_even(10));
    println!("Factorial  of 5 is  ::  {}", factorial::factorial(5));
    println!(
        "Area of circle is {} if r = 5   ",
        shapes::calculate_area(shapes::Shape::Circle(5.0))
    );
    println!(
        "Area of Rectangle is {} if width  = 5 , height = 7.6   ",
        shapes::calculate_area(shapes::Shape::Rectangle(5.0, 7.6))
    );
    let p = person::Person::new("Swap", 21);
    p.say_hello();

    let s = String::from("Birthday");
    print_length::print_length(&s);
    println!("Is im owner of {} :: {}", s, !s.is_empty())
}
