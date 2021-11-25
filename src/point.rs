use std::cmp::PartialEq;
use std::ops::Add;
use crate::field::FieldElement;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldPoint {
    pub x: FieldElement,
    pub y: FieldElement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EccPoint {
    field_point: Option<FieldPoint>, // None if infinity
    a: FieldElement,
    b: FieldElement,
}

impl EccPoint {
    pub fn new(field_point: Option<FieldPoint>, a: FieldElement, b: FieldElement) -> Result<Self, String> {
        // EccPoint is infinity if FieldPoint is none
        if let Some(point) = field_point.clone() {
            // y^2 = x^3 + ax + b
            if point.y.pow_u32(2) != point.x.pow_u32(3) + a*point.x + b {
                let msg = format!("({}, {}) is not on the curve", point.x, point.y);
                return Err(msg);
            }
        }

        Ok(Self { field_point, a, b })
    }
}

impl Add for EccPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("two point cannot be added which not on the same curve");
        }
        if self.field_point.is_none() {
            return other;
        }
        if other.field_point.is_none() {
            return self;
        }

        let (self_point, other_point) = ( self.field_point.unwrap(), other.field_point.unwrap() );
        if self_point.x == other_point.x && self_point.y != other_point.y {
            return Self {
                field_point: None,
                a: self.a,
                b: self.b,
            };
        }

        panic!("not impl within Ecc.add");
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldPoint, EccPoint};
    use crate::field::{FieldElement, FieldElementCreator};
    use crate::prime::Prime;

    const CREATOR13: FieldElementCreator = FieldElementCreator(Prime(13));

    fn get_a_and_b_of_curve() -> (FieldElement, FieldElement) {
        (
            CREATOR13.from_u32(5),
            CREATOR13.from_u32(7)
        )
    }

    #[test]
    fn create_ecc_point_with_none() {
        let (a, b) = get_a_and_b_of_curve();
        EccPoint::new(None, a, b).unwrap();
    }

    #[test]
    fn create_ecc_point_with_point() {
        let (a, b) = get_a_and_b_of_curve();
        let x = CREATOR13.from_i64(-1);
        let y = CREATOR13.from_i64(-1);
        let point = FieldPoint { x, y };
        EccPoint::new(Some(point), a, b).unwrap();
    }

    #[test]
    #[should_panic]
    fn create_ecc_point_fail() {
        let (a, b) = get_a_and_b_of_curve();
        let x = CREATOR13.from_i64(-1);
        let y = CREATOR13.from_i64(-2);
        let point = FieldPoint { x, y };
        EccPoint::new(Some(point), a, b).unwrap();
    }

    #[test]
    fn ecc_point_add_inifity() {
        let (a, b) = get_a_and_b_of_curve();
        let point = Some(FieldPoint { x: CREATOR13.from_i64(-1), y: CREATOR13.from_i64(1) });
        let ecc_point1 = EccPoint::new(point, a, b).unwrap();
        let ecc_point2 = EccPoint::new(None, a, b).unwrap();

        let ecc_point_result = ecc_point1.clone() + ecc_point2.clone();
        assert_eq!(ecc_point1, ecc_point_result);
    }

    #[test]
    fn ecc_point_add_by_using_two_point_with_same_x_and_different_y() {
        let (a, b) = get_a_and_b_of_curve();

        let point1 = Some(FieldPoint { x: CREATOR13.from_i64(-1), y: CREATOR13.from_i64(1) });
        let ecc_point1 = EccPoint::new(point1, a, b).unwrap();

        let point2 = Some(FieldPoint { x: CREATOR13.from_i64(-1), y: CREATOR13.from_i64(-1) });
        let ecc_point2 = EccPoint::new(point2, a, b).unwrap();

        let ecc_point_infinity = EccPoint::new(None, a, b).unwrap();
        let ecc_point_result = ecc_point1 + ecc_point2;

        assert_eq!(ecc_point_infinity, ecc_point_result);
    }
}
