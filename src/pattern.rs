use crate::colour::Colour;
use crate::matrix::Matrix;
use crate::vec3::TypedVec;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PatternType {
    Stripe,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pattern {
    a: Colour,
    b: Colour,
    is: PatternType,
    pub(crate) transform: Option<Matrix<f64>>,
}

impl Pattern {
    pub fn new(is: PatternType, a: Colour, b: Colour) -> Self {
        Pattern {
            a,
            b,
            is,
            transform: None,
        }
    }

    pub fn at(&self, point: TypedVec) -> Colour {
        match self.is {
            PatternType::Stripe => self.stripe_at(point),
        }
    }

    pub fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }

    pub fn set_transform(&mut self, t: Matrix<f64>) {
        self.transform = Some(t);
    }

    fn stripe_at(&self, point: TypedVec) -> Colour {
        if point.x.floor() % 2f64 == 0f64 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod test {
    use crate::colour::*;
    use crate::pattern::Pattern;
    use crate::pattern::PatternType::Stripe;
    use crate::vec3::TypedVec;

    #[test]
    fn test_stripe_constant_y() {
        let s = Pattern::new(Stripe, *WHITE, *BLACK);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 1f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 2f64, 0f64)), *WHITE);
    }

    #[test]
    fn test_stripe_constant_z() {
        let s = Pattern::new(Stripe, *WHITE, *BLACK);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 1f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 2f64)), *WHITE);
    }

    #[test]
    fn test_stripe_alternates_x() {
        let s = Pattern::new(Stripe, *WHITE, *BLACK);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0.9f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(-0.1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(-1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(-1.1f64, 0f64, 0f64)), *WHITE);
    }
}
