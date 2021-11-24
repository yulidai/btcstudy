use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq)]
pub struct EccPoint {
    point: Option<Point>,
    a: i32,
    b: i32,
}

impl EccPoint {
    pub fn new(point: Option<Point>, a: i32, b: i32) -> Result<Self, String> {
        // EccPoint is infinity if Point is none
        if let Some(point_real) = point.clone() {
            // y^2 = x^3 + ax + b
            if point_real.y.pow(2) != point_real.x.pow(3) + a*point_real.x + b {
                let msg = format!("({}, {}) is not on the curve", point_real.x, point_real.y);
                return Err(msg);
            }
        }

        Ok(Self { point, a, b })
    }
}

impl Add for EccPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("two point cannot be added which not on the same curve");
        }
        if self.point.is_none() {
            return other;
        }
        if other.point.is_none() {
            return self;
        }

        let self_point = self.point.unwrap();
        let other_point = other.point.unwrap();
        if self_point.x == other_point.x && self_point.y != other_point.y {
            return Self {
                point: None,
                a: self.a,
                b: self.b,
            };
        }

        panic!("not impl within Ecc.add");
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

    #[test]
    fn ecc_point_add_inifity() {
        let point = Some(Point { x: -1, y: 1 });
        let ecc_point1 = EccPoint::new(point, 5, 7).unwrap();
        let ecc_point2 = EccPoint::new(None, 5, 7).unwrap();

        let ecc_point_result1 = ecc_point1.clone() + ecc_point2.clone();
        let ecc_point_result2 = ecc_point2.clone() + ecc_point1.clone();
        assert!(ecc_point1 == ecc_point_result1);
        assert!(ecc_point1 == ecc_point_result2);
    }

    #[test]
    fn ecc_point_add_by_using_two_point_with_same_x_and_different_y() {
        let point1 = Some(Point { x: -1, y: 1 });
        let ecc_point1 = EccPoint::new(point1, 5, 7).unwrap();

        let point2 = Some(Point { x: -1, y: -1 });
        let ecc_point2 = EccPoint::new(point2, 5, 7).unwrap();

        let ecc_point_infinity = EccPoint::new(None, 5, 7).unwrap();
        let ecc_point_result = ecc_point1 + ecc_point2;
        assert!(ecc_point_infinity == ecc_point_result);
    }
}
