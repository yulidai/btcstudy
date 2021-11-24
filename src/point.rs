pub struct Point {
    x: i32,
    y: i32,
    a: i32,
    b: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Result<Self, String> {
        // y^2 = x^3 + ax + b
        if y.pow(2) != x.pow(3) + a*x + b {
            let msg = format!("({}, {}) is not on the curve", x, y);
            return Err(msg);
        }
        Ok(Self { x, y, a, b })
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn create_point_success() {
        Point::new(-1, -1, 5, 7).unwrap();
    }

    #[test]
    #[should_panic]
    fn create_point_fail() {
        Point::new(-1, -2, 5, 7).unwrap();
    }
}
