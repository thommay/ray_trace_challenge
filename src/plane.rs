use crate::intersection::Intersection;
use crate::intersection::EPSILON;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::{Hittable, HittableImpl};
use crate::vec3::TypedVec;
use anyhow::Result;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Plane {
    pub material: Material,
    pub transform: Option<Matrix<f64>>,
}

impl Plane {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut ret = Vec::new();
        if ray.direction.y.abs() < EPSILON {
            return ret;
        }
        let ray = if let Some(transform) = &self.transform {
            let t = transform.inverse().unwrap();
            ray.transform(&t)
        } else {
            ray
        };
        ret.push(Intersection::new(-ray.origin.y / ray.direction.y, self));
        ret
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: None,
            material: Material::default(),
        }
    }
}

impl Hittable for Plane {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.local_intersect(ray)
    }
    fn normal_at(&self, _p: TypedVec) -> Result<TypedVec> {
        Ok(TypedVec::vector(0f64, 1f64, 0f64))
    }
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }
}

impl Hittable for &Plane {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.local_intersect(ray)
    }

    fn normal_at(&self, _p: TypedVec) -> Result<TypedVec> {
        Ok(TypedVec::vector(0f64, 1f64, 0f64))
    }
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }
}

impl HittableImpl for Plane {}
impl HittableImpl for &Plane {}

#[cfg(test)]
mod test {
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::sphere::Hittable;
    use crate::vec3::TypedVec;

    #[test]
    fn test_intersect_parallel_ray() {
        let p = Plane::default();
        let r = Ray::new(
            TypedVec::point(0f64, 10f64, 0f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let xs = p.intersect(r);
        assert!(xs.is_empty())
    }

    #[test]
    fn test_intersect_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let xs = p.intersect(r);
        assert!(xs.is_empty())
    }

    #[test]
    fn test_intersect_from_above() {
        let p = Plane::default();
        let r = Ray::new(
            TypedVec::point(0f64, 1f64, 0f64),
            TypedVec::vector(0f64, -1f64, 0f64),
        );
        let xs = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1f64);
    }

    #[test]
    fn test_intersect_from_below() {
        let p = Plane::default();
        let r = Ray::new(
            TypedVec::point(0f64, -1f64, 0f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        let xs = p.intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1f64);
    }
}
