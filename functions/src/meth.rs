pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Can't divide with zero , are u doing meth"));
    } else {
        Ok(a / b)
    }
}
