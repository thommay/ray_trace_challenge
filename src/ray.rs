use crate::matrix::Matrix;
use crate::vec3::TypedVec;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: TypedVec,
    pub direction: TypedVec,
}

impl Ray {
    pub fn new(origin: TypedVec, direction: TypedVec) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> TypedVec {
        self.origin + self.direction * time
    }

    pub(crate) fn transform(&self, transform: &'_ Matrix<f64>) -> Self {
        Self {
            direction: { transform.clone() * self.direction },
            origin: { transform * self.origin },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::vec3::TypedVec;

    #[test]
    fn test_position() {
        let start = TypedVec::point(2f64, 3f64, 4f64);
        let r = Ray::new(start, TypedVec::vector(1f64, 0f64, 0f64));
        assert_eq!(r.position(0f64), start);
        assert_eq!(r.position(1f64), TypedVec::point(3f64, 3f64, 4f64));
        assert_eq!(r.position(-1f64), TypedVec::point(1f64, 3f64, 4f64));
        assert_eq!(r.position(2.5), TypedVec::point(4.5, 3f64, 4f64));
    }

    #[test]
    fn test_translate() {
        let r = Ray::new(
            TypedVec::point(1f64, 2f64, 3f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        let m = Matrix::translation(3f64, 4f64, 5f64);
        let out = r.transform(&m);
        assert_eq!(out.origin, TypedVec::point(4f64, 6f64, 8f64));
        assert_eq!(out.direction, TypedVec::vector(0f64, 1f64, 0f64));
    }

    #[test]
    fn test_scale() {
        let r = Ray::new(
            TypedVec::point(1f64, 2f64, 3f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        let m = Matrix::scaling(2f64, 3f64, 4f64);
        let out = r.transform(&m);
        assert_eq!(out.origin, TypedVec::point(2f64, 6f64, 12f64));
        assert_eq!(out.direction, TypedVec::vector(0f64, 3f64, 0f64));
    }
}
