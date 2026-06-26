pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-32.0)*5.0/9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c*5.0/9.0+32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fahrenheit_to_celsius(-459.67), -273.15);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }
}
