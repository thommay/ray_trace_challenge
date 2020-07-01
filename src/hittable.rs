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

pub trait HittableImpl {
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

impl<'a, 'b> PartialEq<dyn Hittable + 'b> for dyn Hittable + 'a {
    fn eq(&self, other: &dyn Hittable) -> bool {
        self.material() == other.material() && self.transform() == other.transform()
    }
}

impl<'a, 'b> PartialOrd<dyn Hittable + 'b> for dyn Hittable + 'a {
    fn partial_cmp(&self, other: &dyn Hittable) -> Option<Ordering> {
        self.material().partial_cmp(other.material())
    }
}

pub trait Hittable: HittableImpl + Debug {}
impl<'a, T> Hittable for T where T: HittableImpl + Debug {}

impl<'a, T> HittableImpl for &T
where
    T: HittableImpl + Debug,
{
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        (*self).intersect(ray)
    }

    fn normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        (*self).normal_at(p)
    }

    fn material(&self) -> &Material {
        (*self).material()
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        (*self).transform()
    }
}
