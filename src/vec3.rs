use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
enum VecType {
    Vector,
    Point,
    Colour,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TypedVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    is: VecType,
}

impl std::fmt::Display for TypedVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn clamp(val: f32) -> f32 {
            if val < 0.0 {
                0.0
            } else if val > 1.0 {
                1.0
            } else {
                val
            }
        }

        write!(
            f,
            "{} {} {}",
            (clamp(self.x) * 255f32).round(),
            (clamp(self.y) * 255f32).round(),
            (clamp(self.z) * 255f32).round()
        )
    }
}
impl TypedVec {
    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            is: VecType::Point,
        }
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            is: VecType::Vector,
        }
    }
    pub fn colour(red: f32, green: f32, blue: f32) -> Self {
        Self {
            x: red,
            y: green,
            z: blue,
            is: VecType::Colour,
        }
    }

    pub fn is_point(&self) -> bool {
        self.is == VecType::Point
    }

    pub fn is_vector(&self) -> bool {
        self.is == VecType::Vector
    }

    pub fn is_colour(&self) -> bool {
        self.is == VecType::Colour
    }

    pub fn magnitude(&self) -> f32 {
        let val = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        val.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            is: self.is,
        }
    }

    pub fn dot_product(&self, rhs: Self) -> f32 {
        assert!(self.is_vector() && rhs.is_vector());
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross_product(&self, rhs: Self) -> Self {
        assert!(self.is_vector() && rhs.is_vector());
        Self::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add for TypedVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let is = if self.is_point() && rhs.is_point() {
            panic!("can't add two points");
        } else if (self.is_point() && rhs.is_vector()) || (self.is_vector() && rhs.is_point()) {
            VecType::Point
        } else {
            VecType::Vector
        };

        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            is,
        }
    }
}

impl Sub for TypedVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let is = if self.is_point() && rhs.is_point() {
            panic!("can't subtract two points");
        } else if (self.is_point() && rhs.is_vector()) || (self.is_vector() && rhs.is_point()) {
            VecType::Point
        } else {
            VecType::Vector
        };

        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            is,
        }
    }
}

impl Neg for TypedVec {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            is: self.is,
        }
    }
}

impl Mul<f32> for TypedVec {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            is: self.is,
        }
    }
}

impl Div<f32> for TypedVec {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            is: self.is,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::{TypedVec, VecType};

    #[test]
    fn test_is_point() {
        let t = TypedVec {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            is: VecType::Point,
        };
        assert_eq!(t.is_point(), true)
    }

    #[test]
    fn test_is_vector() {
        let t = TypedVec {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            is: VecType::Vector,
        };
        assert_eq!(t.is_vector(), true)
    }

    #[test]
    fn test_add() {
        let t = TypedVec {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            is: VecType::Vector,
        } + TypedVec {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            is: VecType::Vector,
        };
        assert_eq!(
            t,
            TypedVec {
                x: 2.0,
                y: 2.0,
                z: 2.0,
                is: VecType::Vector,
            }
        )
    }

    #[test]
    fn test_sub() {
        let t = TypedVec::point(3.0, 2.0, 1.0) - TypedVec::vector(5.0, 6.0, 7.0);
        assert_eq!(
            t,
            TypedVec {
                x: -2.0,
                y: -4.0,
                z: -6.0,
                is: VecType::Point,
            }
        );
        assert_eq!(t.is_point(), true)
    }

    #[test]
    fn test_neg() {
        let t = TypedVec::point(3.0, 2.0, 1.0);
        assert_eq!(-t, TypedVec::point(-3.0, -2.0, -1.0))
    }

    #[test]
    fn test_magnitude() {
        let t = TypedVec::vector(1.0, 2.0, 3.0);
        assert_eq!(t.magnitude(), 14.0_f32.sqrt())
    }

    #[test]
    fn test_negative_magnitude() {
        let t = TypedVec::vector(-1.0, -2.0, -3.0);
        assert_eq!(t.magnitude(), 14.0_f32.sqrt())
    }

    #[test]
    fn test_normalize() {
        let v = TypedVec::vector(1.0, 2.0, 3.0);
        let res = TypedVec::vector(
            1.0 / (14.0_f32.sqrt()),
            2.0 / (14.0_f32.sqrt()),
            3.0 / (14.0_f32.sqrt()),
        );
        assert_eq!(v.normalize(), res)
    }

    #[test]
    fn test_normalize_magnitude() {
        let v = TypedVec::vector(1.0, 2.0, 3.0).normalize();
        assert_eq!(v.magnitude().round(), 1.0)
    }

    #[test]
    fn test_dot_product() {
        let v = TypedVec::vector(1.0, 2.0, 3.0);
        let r = TypedVec::vector(2.0, 3.0, 4.0);
        assert_eq!(v.dot_product(r).round(), 20.0)
    }

    #[test]
    fn test_cross_product() {
        let v = TypedVec::vector(1.0, 2.0, 3.0);
        let r = TypedVec::vector(2.0, 3.0, 4.0);
        assert_eq!(
            v.cross_product(r.clone()),
            TypedVec::vector(-1.0, 2.0, -1.0)
        );
        assert_eq!(r.cross_product(v.clone()), TypedVec::vector(1.0, -2.0, 1.0));
    }
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TypedVec::colour(1.0, 0.0, 0.0)), "255 0 0");
        assert_eq!(
            format!("{}", TypedVec::colour(1.0, 0.8, 0.6)),
            "255 204 153"
        );
    }
}
