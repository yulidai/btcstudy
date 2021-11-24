#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct EccPoint {
    point: Option<Point>,
    a: i32,
    b: i32,
}

impl EccPoint {
    pub fn new(point: Option<Point>, a: i32, b: i32) -> Result<Self, String> {
        if let Some(point_real) = point.clone() {
            // y^2 = x^3 + ax + b
            if point_real.y.pow(2) != point_real.x.pow(3) + a*point_real.x + b {
                let msg = format!("({}, {}) is not on the curve", point_real.x, point_real.y);
                return Err(msg);
            }
        }

        // EccPoint is infinity if Point is none
        Ok(Self { point, a, b })
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, EccPoint};

    #[test]
    fn create_ecc_point_with_none() {
        EccPoint::new(None, 5, 7).unwrap();
    }

    #[test]
    fn create_ecc_point_with_point() {
        let point = Point { x: -1, y: -1 };
        EccPoint::new(Some(point), 5, 7).unwrap();
    }

    #[test]
    #[should_panic]
    fn create_ecc_point_fail() {
        let point = Point { x: -1, y: -2 };
        EccPoint::new(Some(point), 5, 7).unwrap();
    }
}
