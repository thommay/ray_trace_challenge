use crate::colour::Colour;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::pattern::Pattern;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use anyhow::Result;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Sphere {
    pub transform: Option<Matrix<f64>>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: None,
            material: Material::default(),
        }
    }
}
impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn glass() -> Self {
        Self {
            material: {
                let mut m = Material::default();
                m.transparency = 1.0;
                m.refractive_index = 1.5;
                m
            },
            ..Default::default()
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<f64>) {
        self.transform = Some(transform);
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

pub trait Hittable {
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn normal_at(&self, p: TypedVec) -> Result<TypedVec>;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Option<Matrix<f64>>;

    fn pattern_at(&self, pattern: &Pattern, point: TypedVec) -> Result<Colour> {
        let object_point = if let Some(t) = self.transform() {
            t.inverse()? * point
        } else {
            point
        };
        let world_point = if let Some(p) = pattern.transform() {
            p.inverse()? * object_point
        } else {
            object_point
        };
        Ok(pattern.at(world_point))
    }
}

impl<'a, 'b> PartialEq<dyn HittableImpl + 'b> for dyn HittableImpl + 'a {
    fn eq(&self, other: &dyn HittableImpl) -> bool {
        self.material() == other.material() && self.transform() == other.transform()
    }
}

impl<'a, 'b> PartialOrd<dyn HittableImpl + 'b> for dyn HittableImpl + 'a {
    fn partial_cmp(&self, other: &dyn HittableImpl) -> Option<Ordering> {
        self.material().partial_cmp(other.material())
    }
}

pub trait HittableImpl: Hittable + Debug {}
impl HittableImpl for Sphere {}
impl HittableImpl for &Sphere {}

impl Hittable for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut ret = Vec::new();
        let ray = if let Some(transform) = &self.transform {
            let t = transform.inverse().unwrap();
            ray.transform(&t)
        } else {
            ray
        };
        let sphere_to_ray = ray.origin - TypedVec::point(0.0, 0.0, 0.0);
        let a: f64 = ray.direction.dot_product(ray.direction);
        let b: f64 = 2.0 * ray.direction.dot_product(sphere_to_ray);
        let c: f64 = sphere_to_ray.dot_product(sphere_to_ray) - 1.0;
        let d = b.powi(2) - 4.0 * a * c;
        if d < 0.0 {
            return ret;
        }
        ret.push(Intersection::new((-b - d.sqrt()) / (2.0 * a), self));
        ret.push(Intersection::new((-b + d.sqrt()) / (2.0 * a), self));
        ret
    }

    fn normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        let c = TypedVec::point(0f64, 0f64, 0f64);
        if let Some(transform) = &self.transform {
            let object_point = transform.inverse()? * p;
            let object_normal = object_point - c;
            let mut world_normal = transform.inverse()?.transpose() * object_normal;
            world_normal.w = 0f64;
            Ok(world_normal.normalize())
        } else {
            Ok((p - c).normalize())
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }
}

impl Hittable for &Sphere {
    fn intersect<'a>(&self, ray: Ray) -> Vec<Intersection> {
        let mut ret = Vec::new();
        let ray = if let Some(transform) = &self.transform {
            let t = transform.inverse().unwrap();
            ray.transform(&t)
        } else {
            ray
        };
        let sphere_to_ray = ray.origin - TypedVec::point(0.0, 0.0, 0.0);
        let a: f64 = ray.direction.dot_product(ray.direction);
        let b: f64 = 2.0 * ray.direction.dot_product(sphere_to_ray);
        let c: f64 = sphere_to_ray.dot_product(sphere_to_ray) - 1.0;
        let d = b.powi(2) - 4.0 * a * c;
        if d < 0.0 {
            return ret;
        }
        ret.push(Intersection::new((-b - d.sqrt()) / (2.0 * a), self));
        ret.push(Intersection::new((-b + d.sqrt()) / (2.0 * a), self));
        ret
    }

    fn normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        let c = TypedVec::point(0f64, 0f64, 0f64);
        if let Some(transform) = &self.transform {
            let object_point = transform.inverse()? * p;
            let object_normal = object_point - c;
            let mut world_normal = transform.inverse()?.transpose() * object_normal;
            world_normal.w = 0f64;
            Ok(world_normal.normalize())
        } else {
            Ok((p - c).normalize())
        }
    }
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }
}

#[cfg(test)]
mod test {
    use crate::colour::*;
    use crate::matrix::{Axis, Matrix};
    use crate::pattern::Pattern;
    use crate::pattern::PatternType::Stripe;
    use crate::ray::Ray;
    use crate::sphere::{Hittable, Sphere};
    use crate::vec3::TypedVec;

    #[test]
    fn test_intersect() {
        let r = Ray::new(
            TypedVec::point(0.0, 0.0, -5.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_tangent() {
        let r = Ray::new(
            TypedVec::point(0.0, 1.0, -5.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn test_missing() {
        let r = Ray::new(
            TypedVec::point(0.0, 2.0, -5.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_behind() {
        let r = Ray::new(
            TypedVec::point(0.0, 0.0, 5.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_inside() {
        let r = Ray::new(
            TypedVec::point(0.0, 0.0, 0.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_default_transform() {
        let r = Ray::new(
            TypedVec::point(0.0, 0.0, -5.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_translated_sphere() {
        let r = Ray::new(
            TypedVec::point(0.0, 0.0, -5.0),
            TypedVec::vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_normal_x() {
        let s = Sphere::new();
        let n = s.normal_at(TypedVec::point(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(n, TypedVec::vector(1f64, 0f64, 0f64));
    }
    #[test]
    fn test_normal_y() {
        let s = Sphere::new();
        let n = s.normal_at(TypedVec::point(0.0, 1.0, 0.0)).unwrap();
        assert_eq!(n, TypedVec::vector(0f64, 1f64, 0f64));
    }
    #[test]
    fn test_normal_z() {
        let s = Sphere::new();
        let n = s.normal_at(TypedVec::point(0.0, 0.0, 1.0)).unwrap();
        assert_eq!(n, TypedVec::vector(0f64, 0f64, 1f64));
    }
    #[test]
    fn test_normal_nonaxial() {
        let s = Sphere::new();
        let v = 3f64.sqrt() / 3.0;
        let n = s.normal_at(TypedVec::point(v, v, v)).unwrap();
        assert_eq!(n, TypedVec::vector(v, v, v));
    }

    #[test]
    fn test_normal_normalized() {
        let s = Sphere::new();
        let v = 3f64.sqrt() / 3.0;
        let p = TypedVec::point(v, v, v);
        let n = s.normal_at(p).unwrap();
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_normal_translated() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0f64, 1f64, 0f64));
        let n = s
            .normal_at(TypedVec::point(0.0, 1.70711, -0.70711))
            .unwrap();
        assert_eq!(
            n.round(100000f64),
            TypedVec::vector(0f64, 0.70711, -0.70711)
        );
    }

    #[test]
    fn test_normal_transformed() {
        let mut s = Sphere::new();
        let m = Matrix::scaling(1f64, 0.5, 1f64)
            * Matrix::rotation(Axis::Z, std::f64::consts::PI / 5f64);
        s.set_transform(m);
        let v = 2f64.sqrt() / 2.0;
        let p = TypedVec::point(0f64, v, -v);
        let n = s.normal_at(p).unwrap();
        assert_eq!(
            n.round(100000f64),
            TypedVec::vector(0f64, 0.97014, -0.24254)
        );
    }

    #[test]
    fn test_stripe_object_transform() {
        let mut s = Sphere::new();
        s.transform = Some(Matrix::scaling(2f64, 2f64, 2f64));
        let p = Pattern::new(Stripe, *WHITE, *BLACK, false);
        let c = s.pattern_at(&p, TypedVec::point(1.5, 0f64, 0f64)).unwrap();
        assert_eq!(c, *WHITE)
    }

    #[test]
    fn test_stripe_pattern_transform() {
        let s = Sphere::new();
        let mut p = Pattern::new(Stripe, *WHITE, *BLACK, false);
        p.transform = Some(Matrix::scaling(2f64, 2f64, 2f64));
        let c = s.pattern_at(&p, TypedVec::point(1.5, 0f64, 0f64)).unwrap();
        assert_eq!(c, *WHITE)
    }

    #[test]
    fn test_stripe_object_pattern_transform() {
        let mut s = Sphere::new();
        s.transform = Some(Matrix::scaling(2f64, 2f64, 2f64));
        let mut p = Pattern::new(Stripe, *WHITE, *BLACK, false);
        p.transform = Some(Matrix::translation(0.5f64, 0f64, 0f64));
        let c = s.pattern_at(&p, TypedVec::point(2.5, 0f64, 0f64)).unwrap();
        assert_eq!(c, *WHITE)
    }
}
