use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VecType {
    Vector,
    Point,
    Colour,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TypedVec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub(crate) w: f64,
    pub(crate) is: VecType,
}

impl std::fmt::Display for TypedVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn clamp(val: f64) -> f64 {
            if val < 0f64 {
                0.0
            } else if val > 1f64 {
                1.0
            } else {
                val.into()
            }
        }

        write!(
            f,
            "{} {} {}",
            (clamp(self.x) * 255f64).round(),
            (clamp(self.y) * 255f64).round(),
            (clamp(self.z) * 255f64).round()
        )
    }
}
impl TypedVec {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 1f64,
            is: VecType::Point,
        }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 0f64,
            is: VecType::Vector,
        }
    }
    pub fn colour(red: f64, green: f64, blue: f64) -> Self {
        Self {
            x: red,
            y: green,
            z: blue,
            w: 0f64,
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

    pub fn magnitude(&self) -> f64 {
        let val = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        val.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w,
            is: self.is,
        }
    }

    pub fn reflect(&self, rhs: Self) -> Self {
        assert!(self.is_vector() && rhs.is_vector());
        *self - rhs * 2f64 * self.dot_product(rhs)
    }

    pub fn dot_product(&self, rhs: Self) -> f64 {
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

    #[cfg(test)]
    pub(crate) fn round(&self, factor: f64) -> Self {
        Self {
            is: self.is,
            w: self.w,
            x: { (self.x * factor).round() / factor },
            y: { (self.y * factor).round() / factor },
            z: { (self.z * factor).round() / factor },
        }
    }
}

impl Add for TypedVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (is, w) = if self.is_point() && rhs.is_point() {
            panic!("can't add two points");
        } else if (self.is_point() && rhs.is_vector()) || (self.is_vector() && rhs.is_point()) {
            (VecType::Point, 1f64)
        } else {
            (VecType::Vector, 0f64)
        };

        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w,
            is,
        }
    }
}

impl Sub for TypedVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (is, w) = if self.is_point() && rhs.is_vector() {
            (VecType::Point, 1f64)
        } else if self.is_vector() && rhs.is_point() {
            panic!("Subtracting a point from a vector makes no sense");
        } else {
            (VecType::Vector, 0f64)
        };

        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w,
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
            w: self.w,
            is: self.is,
        }
    }
}

impl Mul<f64> for TypedVec {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w,
            is: self.is,
        }
    }
}

impl Div<f64> for TypedVec {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w,
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
            w: 1f64,
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
            w: 0f64,
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
            w: 0f64,
            is: VecType::Vector,
        } + TypedVec {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 0f64,
            is: VecType::Vector,
        };
        assert_eq!(
            t,
            TypedVec {
                x: 2.0,
                y: 2.0,
                z: 2.0,
                w: 0f64,
                is: VecType::Vector,
            }
        )
    }

    #[test]
    fn test_sub_point_vec() {
        let t = TypedVec::point(3.0, 2.0, 1.0) - TypedVec::vector(5.0, 6.0, 7.0);
        assert_eq!(
            t,
            TypedVec {
                x: -2.0,
                y: -4.0,
                z: -6.0,
                w: 1f64,
                is: VecType::Point,
            }
        );
        assert_eq!(t.is_point(), true)
    }

    #[test]
    fn test_sub_point_point() {
        let t = TypedVec::point(3.0, 2.0, 1.0) - TypedVec::point(5.0, 6.0, 7.0);
        assert_eq!(
            t,
            TypedVec {
                x: -2.0,
                y: -4.0,
                z: -6.0,
                w: 0f64,
                is: VecType::Vector,
            }
        );
        assert_eq!(t.is_vector(), true)
    }

    #[test]
    fn test_sub_vec_vec() {
        let t = TypedVec::vector(3.0, 2.0, 1.0) - TypedVec::vector(5.0, 6.0, 7.0);
        assert_eq!(
            t,
            TypedVec {
                x: -2.0,
                y: -4.0,
                z: -6.0,
                w: 0f64,
                is: VecType::Vector,
            }
        );
        assert_eq!(t.is_vector(), true)
    }

    #[test]
    fn test_neg() {
        let t = TypedVec::point(3.0, 2.0, 1.0);
        assert_eq!(-t, TypedVec::point(-3.0, -2.0, -1.0))
    }

    #[test]
    fn test_magnitude() {
        let t = TypedVec::vector(1.0, 2.0, 3.0);
        assert_eq!(t.magnitude(), 14.0_f64.sqrt())
    }

    #[test]
    fn test_negative_magnitude() {
        let t = TypedVec::vector(-1.0, -2.0, -3.0);
        assert_eq!(t.magnitude(), 14.0_f64.sqrt())
    }

    #[test]
    fn test_normalize() {
        let v = TypedVec::vector(1.0, 2.0, 3.0);
        let res = TypedVec::vector(
            1.0 / (14.0_f64.sqrt()),
            2.0 / (14.0_f64.sqrt()),
            3.0 / (14.0_f64.sqrt()),
        );
        assert_eq!(v.normalize(), res)
    }

    #[test]
    fn test_normalize_magnitude() {
        let v = TypedVec::vector(1.0, 2.0, 3.0).normalize();
        let m = v.magnitude();
        assert_eq!(m.round(), 1.0)
    }

    #[test]
    fn test_dot_product() {
        let v = TypedVec::vector(1.0, 2.0, 3.0);
        let r = TypedVec::vector(2.0, 3.0, 4.0);
        let m = v.dot_product(r);
        assert_eq!(m.round(), 20.0)
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
    fn test_reflect_45() {
        let v = TypedVec::vector(1.0, -1.0, 0.0);
        let n = TypedVec::vector(0.0, 1.0, 0.0);
        let r = TypedVec::vector(1.0, 1.0, 0.0);
        assert_eq!(v.reflect(n), r)
    }

    #[test]
    fn test_reflect_slant() {
        let v = TypedVec::vector(0.0, -1.0, 0.0);
        let s = 2f64.sqrt() / 2f64;
        let n = TypedVec::vector(s, s, 0.0);
        let r = TypedVec::vector(1.0, 0.0, 0.0);
        assert_eq!(v.reflect(n).round(1000f64), r)
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
