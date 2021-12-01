use primitive_types::U256;
use std::cmp::PartialEq;
use std::ops::{Add, Mul};
use crate::field::{FieldElement, FieldElementCreator};

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
            if point.y.pow_u256(2.into()) != point.x.pow_u256(3.into()) + a*point.x + b {
                let msg = format!("({:x}, {:x}) is not on the curve({},{})", point.x, point.y, a, b);
                return Err(msg);
            }
        }

        Ok(Self { field_point, a, b })
    }

    pub fn is_infinity(&self) -> bool {
        self.field_point.is_none()
    }

    pub fn field_point(&self) -> &Option<FieldPoint> {
        &self.field_point
    }

    pub fn into_field_point(self) -> Option<FieldPoint> {
        self.field_point
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
        let (x1, y1) = (self_point.x, self_point.y);
        let (x2, y2) = (other_point.x, other_point.y);

        // x1 != x2
        if x1 != x2 {
            let s = (y2 - y1) / (x2 - x1); // div will be convert into field_element div
            let x3 = s.pow_u256(2.into()) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;

            let point = FieldPoint { x: x3, y: y3 };
            return Self {
                field_point: Some(point),
                a: self.a,
                b: self.b,
            };
        }

        // x1 == x2 && y1 != y2
        if self_point.y != other_point.y {
            return Self {
                field_point: None,
                a: self.a,
                b: self.b,
            };
        }

        // x1 == x2 && y1 == y2 && y1 == 0
        if y1.is_zero() {
            return Self {
                field_point: None,
                a: self.a,
                b: self.b,
            }
        }

        // x1 == x2 && y1 == y2 && y1 != 0
        let element_creator = FieldElementCreator(self.a.prime());
        let two = element_creator.from_u256(2.into());
        let three = element_creator.from_u256(3.into());
        let s = (three * x1.pow_u256(2.into()) + self.a) / (two * y1);
        let x3 = s.pow_u256(2.into()) - two * x1;
        let y3 = s * (x1 - x3) - y1;

        let point = FieldPoint { x: x3, y: y3 };
        Self {
            field_point: Some(point),
            a: self.a,
            b: self.b,
        }
    }
}

// point * coefficient
impl Mul<U256> for EccPoint {
    type Output = Self;

    fn mul(self, mut coefficient: U256) -> Self {
        let mut result = EccPoint {
            field_point: None,
            a: self.a,
            b: self.b
        };
        let mut current = self;

        while coefficient > U256::zero() {
            if coefficient % 2 == U256::one() {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coefficient = coefficient >> 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldPoint, EccPoint};
    use crate::field::{FieldElement, FieldElementCreator};
    use crate::prime::Prime;
    use primitive_types::U256;


    fn get_a_and_b_of_curve() -> (FieldElement, FieldElement) {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        (
            creator13.from_u256(5.into()),
            creator13.from_u256(7.into())
        )
    }

    #[test]
    fn create_ecc_point_with_none() {
        let (a, b) = get_a_and_b_of_curve();
        EccPoint::new(None, a, b).unwrap();
    }

    #[test]
    fn create_ecc_point_with_point() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let (a, b) = get_a_and_b_of_curve();
        let x = creator13.from_i64(-1);
        let y = creator13.from_i64(-1);
        let point = FieldPoint { x, y };
        EccPoint::new(Some(point), a, b).unwrap();
    }

    #[test]
    #[should_panic]
    fn create_ecc_point_fail() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let (a, b) = get_a_and_b_of_curve();
        let x = creator13.from_i64(-1);
        let y = creator13.from_i64(-2);
        let point = FieldPoint { x, y };
        EccPoint::new(Some(point), a, b).unwrap();
    }

    #[test]
    fn ecc_point_add_inifity() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let (a, b) = get_a_and_b_of_curve();

        let point = Some(FieldPoint { x: creator13.from_i64(-1), y: creator13.from_i64(1) });
        let ecc_point1 = EccPoint::new(point, a, b).unwrap();
        let ecc_point2 = EccPoint::new(None, a, b).unwrap();

        let ecc_point_result = ecc_point1.clone() + ecc_point2.clone();
        assert_eq!(ecc_point1, ecc_point_result);
    }

    #[test]
    fn ecc_point_add_by_using_two_point_with_different_x() {
        let creator223 = FieldElementCreator(Prime(U256::from(223)));

        let a = creator223.from_u256(0.into());
        let b = creator223.from_u256(7.into());

        let point1 = Some(FieldPoint { x: creator223.from_i64(170), y: creator223.from_i64(142) });
        let ecc_point1 = EccPoint::new(point1, a, b).unwrap();

        let point2 = Some(FieldPoint { x: creator223.from_i64(60), y: creator223.from_i64(139) });
        let ecc_point2 = EccPoint::new(point2, a, b).unwrap();

        let point_correct = Some(FieldPoint { x: creator223.from_i64(220), y: creator223.from_i64(181) });
        let ecc_point_result_correct = EccPoint::new(point_correct, a, b).unwrap();

        let ecc_point_result = ecc_point1 + ecc_point2;
        assert_eq!(ecc_point_result, ecc_point_result_correct);
    }

    #[test]
    fn ecc_point_add_by_using_two_point_with_same_x_and_different_y() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let (a, b) = get_a_and_b_of_curve();

        let point1 = Some(FieldPoint { x: creator13.from_i64(-1), y: creator13.from_i64(1) });
        let ecc_point1 = EccPoint::new(point1, a, b).unwrap();

        let point2 = Some(FieldPoint { x: creator13.from_i64(-1), y: creator13.from_i64(-1) });
        let ecc_point2 = EccPoint::new(point2, a, b).unwrap();

        let ecc_point_infinity = EccPoint::new(None, a, b).unwrap();
        let ecc_point_result = ecc_point1 + ecc_point2;

        assert_eq!(ecc_point_infinity, ecc_point_result);
    }

    #[test]
    fn ecc_point_add_by_using_two_point_with_same_x_and_same_y_and_y_is_zero() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));

        let a = creator13.from_u256(0.into());
        let b = creator13.from_u256(12.into());

        let point = Some(FieldPoint { x: creator13.from_i64(3), y: creator13.from_i64(0) });
        let ecc_point = EccPoint::new(point, a, b).unwrap();

        let ecc_point_infinity = EccPoint::new(None, a, b).unwrap();
        let ecc_point_result = ecc_point.clone() + ecc_point;

        assert_eq!(ecc_point_infinity, ecc_point_result);
    }

    #[test]
    fn ecc_point_add_by_using_two_point_with_same_x_and_same_y_and_y_is_not_zero() {
        let creator13: FieldElementCreator = FieldElementCreator(Prime(U256::from(13)));

        let a = creator13.from_u256(0.into());
        let b = creator13.from_u256(12.into());

        let point = Some(FieldPoint { x: creator13.from_i64(7), y: creator13.from_i64(11) });
        let ecc_point = EccPoint::new(point, a, b).unwrap();

        let point_result = Some(FieldPoint { x: creator13.from_i64(0), y: creator13.from_i64(8) });
        let ecc_point_correct = EccPoint::new(point_result, a, b).unwrap();
        let ecc_point_result = ecc_point.clone() + ecc_point;

        assert_eq!(ecc_point_correct, ecc_point_result);
    }

    #[test]
    fn ecc_point_mul_coefficient_one() {
        let creator223 = FieldElementCreator(Prime(U256::from(223)));

        let a = creator223.from_u256(0.into());
        let b = creator223.from_u256(7.into());

        let point = Some(FieldPoint { x: creator223.from_i64(15), y: creator223.from_i64(86) });
        let ecc_point = EccPoint::new(point, a, b).unwrap();

        let result = ecc_point.clone() * 1.into();
        assert_eq!(ecc_point, result);
    }

    #[test]
    fn ecc_point_mul_coefficient_two() {
        let creator223 = FieldElementCreator(Prime(U256::from(223)));

        let a = creator223.from_u256(0.into());
        let b = creator223.from_u256(7.into());

        let point = Some(FieldPoint { x: creator223.from_i64(15), y: creator223.from_i64(86) });
        let ecc_point = EccPoint::new(point, a, b).unwrap();
        let result = ecc_point.clone() * 2.into();

        let point_correct = Some(FieldPoint { x: creator223.from_i64(139), y: creator223.from_i64(86) });
        let ecc_point_correct = EccPoint::new(point_correct, a, b).unwrap();

        assert_eq!(ecc_point_correct, result);
    }

    #[test]
    fn ecc_point_mul_coefficient_overflow_1() {
        let creator223 = FieldElementCreator(Prime(U256::from(223)));

        let a = creator223.from_u256(0.into());
        let b = creator223.from_u256(7.into());

        let point = Some(FieldPoint { x: creator223.from_i64(15), y: creator223.from_i64(86) });
        let ecc_point = EccPoint::new(point, a, b).unwrap();
        let result = ecc_point.clone() * 7.into();

        assert!(result.is_infinity());
    }

    #[test]
    fn ecc_point_mul_coefficient_overflow_2() {
        let creator223 = FieldElementCreator(Prime(U256::from(223)));

        let a = creator223.from_u256(0.into());
        let b = creator223.from_u256(7.into());

        let point = Some(FieldPoint { x: creator223.from_i64(15), y: creator223.from_i64(86) });
        let ecc_point = EccPoint::new(point, a, b).unwrap();

        let result = ecc_point.clone() * 8.into();
        assert_eq!(ecc_point, result); // 7 is overflow, 8 % 7 = 1, so is equal
    }
}
