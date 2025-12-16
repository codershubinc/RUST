use crate::printable::Printable;

pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

pub fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w, h) => w * h,
    }
}

impl Printable for Shape {
    fn format(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle with radius {}", r),
            Shape::Rectangle(w, h) => format!("Rectangle {}x{}", w, h),
        }
    }
}
