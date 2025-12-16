mod factorial;
mod is_even;
mod meth;
mod person;
mod print_length;
mod printable;
mod shapes;
mod sum_list;

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
    println!("Is im owner of {} :: {}", s, !s.is_empty());

    let mut lst: Vec<i32> = vec![];
    lst.push(10);
    lst.push(10);
    lst.push(10);
    lst.push(10);

    println!("Sum of the list  lst is {}", sum_list::sum_list(&lst));
    let sh = shapes::Shape::Circle(5.5);
    printable::print_details(&sh);
    printable::print_details(&p);

    let result = meth::divide(2.0, 0.0);

    match result {
        Ok(value) => println!("Do meth of 2/4 is equals {} ", value),
        Err(err) => println!("Are u doing meth {} ", err),
    }
}
