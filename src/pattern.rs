use crate::colour::{Colour, WHITE};
use crate::matrix::Matrix;
use crate::vec3::TypedVec;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PatternType {
    Checker,
    Gradient,
    Ring,
    Stripe,
    None,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pattern {
    a: Colour,
    b: Colour,
    is: PatternType,
    perturb: bool,
    pub(crate) transform: Option<Matrix<f64>>,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            is: PatternType::None,
            transform: Some(Matrix::identity(4)),
            a: *WHITE,
            b: *WHITE,
            perturb: false,
        }
    }
}
impl Pattern {
    pub fn new(is: PatternType, a: Colour, b: Colour, perturb: bool) -> Self {
        Pattern {
            a,
            b,
            is,
            perturb,
            transform: None,
        }
    }

    pub fn checker(a: Colour, b: Colour, perturb: bool) -> Self {
        Pattern {
            a,
            b,
            perturb,
            is: PatternType::Checker,
            transform: None,
        }
    }

    pub fn gradient(a: Colour, b: Colour, perturb: bool) -> Self {
        Pattern {
            a,
            b,
            perturb,
            is: PatternType::Gradient,
            transform: None,
        }
    }

    pub fn ring(a: Colour, b: Colour, perturb: bool) -> Self {
        Pattern {
            a,
            b,
            perturb,
            is: PatternType::Ring,
            transform: None,
        }
    }

    pub fn stripe(a: Colour, b: Colour, perturb: bool) -> Self {
        Pattern {
            a,
            b,
            perturb,
            is: PatternType::Stripe,
            transform: None,
        }
    }

    pub fn at(&self, point: TypedVec) -> Colour {
        match self.is {
            PatternType::Checker => self.checker_at(point),
            PatternType::Gradient => self.gradient_at(point),
            PatternType::Ring => self.ring_at(point),
            PatternType::Stripe => self.stripe_at(point),
            PatternType::None => self.test_pattern_at(point),
        }
    }

    pub fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }

    pub fn set_transform(&mut self, t: Matrix<f64>) {
        self.transform = Some(t);
    }

    fn checker_at(&self, point: TypedVec) -> Colour {
        let (x, y, z) = self.perturb(point);
        if x.floor() + y.floor() + z.floor() % 2f64 == 0f64 {
            self.a
        } else {
            self.b
        }
    }

    fn gradient_at(&self, point: TypedVec) -> Colour {
        let (x, _, _) = self.perturb(point);
        let d = self.b - self.a;
        let f = x - x.floor();
        self.a + d * f
    }

    fn ring_at(&self, point: TypedVec) -> Colour {
        let (x, _, z) = self.perturb(point);
        if (x.powi(2) + z.powi(2)).sqrt().floor() % 2f64 == 0f64 {
            self.a
        } else {
            self.b
        }
    }

    fn stripe_at(&self, point: TypedVec) -> Colour {
        let (x, _, _) = self.perturb(point);
        if x.floor() % 2f64 == 0f64 {
            self.a
        } else {
            self.b
        }
    }

    fn test_pattern_at(&self, point: TypedVec) -> Colour {
        Colour::new(point.x, point.y, point.z)
    }

    fn perturb(&self, point: TypedVec) -> (f64, f64, f64) {
        if !self.perturb {
            return (point.x, point.y, point.z);
        }
        let new_x = point.x + perlin_noise(point.x, point.y, point.z) * 0.01;
        let new_y = point.y + perlin_noise(point.x, point.y, point.z + 1f64) * 0.01;
        let new_z = point.y + perlin_noise(point.x, point.y, point.z + 2f64) * 0.01;
        (new_x, new_y, new_z)
    }
}

lazy_static! {
    static ref P: Vec<usize> = {
    // Hash lookup table as defined by Ken Perlin.  This is a randomly
    // arranged array of all numbers from 0-255 inclusive.
        let mut p = vec![
            151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103,
            30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197,
            62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20,
            125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
            111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54,
            65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135,
            130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124,
            123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17,
            182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153,
            101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178,
            185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
            241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184,
            84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29,
            24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
        ];
        p.append(&mut p.clone());
        p
    };
}

fn perlin_noise(x: f64, y: f64, z: f64) -> f64 {
    let xi = x as usize & 255;
    let yi = y as usize & 255;
    let zi = z as usize & 255;
    let xf = x.fract();
    let yf = y.fract();
    let zf = z.fract();

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6f64 - 15f64) + 10f64)
    }

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let aaa = P[P[P[xi] + yi] + zi];
    let aba = P[P[P[xi] + yi + 1] + zi];
    let aab = P[P[P[xi] + yi] + zi + 1];
    let abb = P[P[P[xi] + yi + 1] + zi + 1];
    let baa = P[P[P[xi + 1] + yi] + zi];
    let bba = P[P[P[xi + 1] + yi + 1] + zi];
    let bab = P[P[P[xi + 1] + yi] + zi + 1];
    let bbb = P[P[P[xi + 1] + yi + 1] + zi + 1];

    fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        let k = if h & 1 == 0 { u } else { -u };
        let p = if h & 2 == 0 { v } else { -v };
        k + p
    }

    fn lerp(a: f64, b: f64, x: f64) -> f64 {
        a + x * (b - a)
    }

    let y1 = lerp(
        lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1f64, yf, zf), u),
        lerp(
            grad(aba, xf, yf - 1f64, zf),
            grad(bba, xf - 1f64, yf - 1f64, zf),
            u,
        ),
        v,
    );
    let y2 = lerp(
        lerp(
            grad(aab, xf, yf, zf - 1f64),
            grad(bab, xf - 1f64, yf, zf - 1f64),
            u,
        ),
        lerp(
            grad(abb, xf, yf - 1f64, zf - 1f64),
            grad(bbb, xf - 1f64, yf - 1f64, zf - 1f64),
            u,
        ),
        v,
    );
    (lerp(y1, y2, w) + 1f64) / 2f64
}

#[cfg(test)]
mod test {
    use crate::colour::*;
    use crate::hittable::HittableImpl;
    use crate::matrix::Matrix;
    use crate::pattern::Pattern;
    use crate::pattern::PatternType::Stripe;
    use crate::sphere::Sphere;
    use crate::vec3::TypedVec;

    #[test]
    fn test_pattern_object_transform() {
        let mut shape = Sphere::default();
        shape.transform = Some(Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::default();
        let c = shape.pattern_at(&pattern, TypedVec::point(2.0, 3.0, 4.0));
        assert_eq!(c.unwrap(), Colour::new(1.0, 1.5, 2.0))
    }

    #[test]
    fn test_pattern_pattern_transform() {
        let shape = Sphere::default();
        let mut pattern = Pattern::default();
        pattern.transform = Some(Matrix::scaling(2.0, 2.0, 2.0));
        let c = shape.pattern_at(&pattern, TypedVec::point(2.0, 3.0, 4.0));
        assert_eq!(c.unwrap(), Colour::new(1.0, 1.5, 2.0))
    }

    #[test]
    fn test_pattern_object_pattern_transform() {
        let mut shape = Sphere::default();
        shape.transform = Some(Matrix::scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::default();
        pattern.transform = Some(Matrix::translation(0.5, 1.0, 1.5));
        let c = shape.pattern_at(&pattern, TypedVec::point(2.5, 3.0, 3.5));
        assert_eq!(c.unwrap(), Colour::new(0.75, 0.5, 0.25))
    }

    #[test]
    fn test_stripe_constant_y() {
        let s = Pattern::new(Stripe, *WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 1f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 2f64, 0f64)), *WHITE);
    }

    #[test]
    fn test_stripe_constant_z() {
        let s = Pattern::new(Stripe, *WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 1f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 2f64)), *WHITE);
    }

    #[test]
    fn test_stripe_alternates_x() {
        let s = Pattern::new(Stripe, *WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0.9f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(-0.1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(-1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(-1.1f64, 0f64, 0f64)), *WHITE);
    }

    #[test]
    fn test_gradient() {
        let p = Pattern::gradient(*WHITE, *BLACK, false);
        assert_eq!(p.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(
            p.at(TypedVec::point(0.25f64, 0f64, 0f64)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            p.at(TypedVec::point(0.5f64, 0f64, 0f64)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            p.at(TypedVec::point(0.75f64, 0f64, 0f64)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn test_ring() {
        let s = Pattern::ring(*WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(1f64, 0f64, 0f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 1f64)), *BLACK);
        assert_eq!(s.at(TypedVec::point(0.708, 0f64, 0.708f64)), *BLACK);
    }

    #[test]
    fn test_checker_x() {
        let s = Pattern::checker(*WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0.99f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(1.1, 0f64, 0f64)), *BLACK);
    }

    #[test]
    fn test_checker_y() {
        let s = Pattern::checker(*WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0.99f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 1.1f64, 0f64)), *BLACK);
    }

    #[test]
    fn test_checker_z() {
        let s = Pattern::checker(*WHITE, *BLACK, false);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 0.99f64)), *WHITE);
        assert_eq!(s.at(TypedVec::point(0f64, 0f64, 1.1f64)), *BLACK);
    }
}
