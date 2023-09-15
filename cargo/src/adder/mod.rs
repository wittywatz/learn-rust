pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_integers() {
        assert_eq!(add(5, 6), 11);
    }

    #[test]
    fn it_adds_floats() {
        let result: f64 = add(5.2, 4.3);
        assert!((result - 9.5).abs() < std::f64::EPSILON);
    }
}
