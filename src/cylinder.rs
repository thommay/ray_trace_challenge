use crate::hittable::{Hittable, HittableImpl};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use crate::{ZeroIsh, EPSILON};
use anyhow::Result;
use std::f64::INFINITY;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Cylinder {
    pub transform: Option<Matrix<f64>>,
    pub material: Material,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            transform: None,
            material: Material::default(),
            closed: false,
        }
    }
}

impl Cylinder {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray = if let Some(transform) = &self.transform {
            let t = transform.inverse().unwrap();
            ray.transform(&t)
        } else {
            ray
        };
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        if a.zeroish() {
            return self.intersect_caps(ray);
        }
        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return vec![];
        }
        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };
        let mut xs = vec![];
        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, self));
        }
        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, self));
        }
        xs.append(&mut self.intersect_caps(ray));
        xs
    }

    fn local_normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        let dist = p.x.powi(2) + p.z.powi(2);
        if dist < 1.0 && p.y >= self.maximum - EPSILON {
            Ok(TypedVec::vector(0.0, 1.0, 0.0))
        } else if dist < 1.0 && p.y <= self.minimum + EPSILON {
            Ok(TypedVec::vector(0.0, -1.0, 0.0))
        } else {
            Ok(TypedVec::vector(p.x, 0.0, p.z))
        }
    }
}

pub trait Capped {
    fn intersect_caps(&self, ray: Ray) -> Vec<Intersection>
    where
        Self: Hittable + std::marker::Sized,
    {
        let mut xs = vec![];
        if !self.closed() || ray.direction.y.zeroish() {
            return xs;
        }

        let t = (self.minimum() - ray.origin.y) / ray.direction.y;
        if self.check_caps(ray, t, self.minimum()) {
            xs.push(Intersection::new(t, self))
        }

        let t = (self.maximum() - ray.origin.y) / ray.direction.y;
        if self.check_caps(ray, t, self.maximum()) {
            xs.push(Intersection::new(t, self))
        }
        xs
    }

    fn closed(&self) -> bool;
    fn minimum(&self) -> f64;
    fn maximum(&self) -> f64;
    fn check_caps(&self, ray: Ray, t: f64, y: f64) -> bool;
}

impl Capped for Cylinder {
    fn check_caps(&self, ray: Ray, t: f64, _: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn closed(&self) -> bool {
        self.closed
    }

    fn minimum(&self) -> f64 {
        self.minimum
    }

    fn maximum(&self) -> f64 {
        self.maximum
    }
}

impl HittableImpl for Cylinder {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.local_intersect(ray)
    }

    fn normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        self.local_normal_at(p)
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
    use crate::cylinder::Cylinder;
    use crate::hittable::HittableImpl;
    use crate::ray::Ray;
    use crate::roundf;
    use crate::vec3::TypedVec;

    #[test]
    fn test_cylinder_miss() {
        let c = Cylinder::default();
        let examples = vec![
            (
                TypedVec::point(1.0, 0.0, 0.0),
                TypedVec::vector(0.0, 1.0, 0.0),
            ),
            (
                TypedVec::point(0.0, 0.0, 0.0),
                TypedVec::vector(0.0, 1.0, 0.0),
            ),
            (
                TypedVec::point(0.0, 0.0, -5.0),
                TypedVec::vector(1.0, 1.0, 1.0),
            ),
        ];
        for e in examples {
            let (o, d) = e;
            let direction = d.normalize();
            let r = Ray::new(o, direction);
            assert_eq!(c.local_intersect(r).len(), 0)
        }
    }

    #[test]
    fn test_cylinder_ray_strike() {
        let c = Cylinder::default();
        let examples = vec![
            (
                "1",
                TypedVec::point(1.0, 0.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                "2",
                TypedVec::point(0.0, 0.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                "3",
                TypedVec::point(0.5, 0.0, -5.0),
                TypedVec::vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];
        for e in examples {
            let (name, o, d, t0, t1) = e;
            dbg!(name);
            let direction = d.normalize();
            let r = Ray::new(o, direction);
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(roundf(xs[0].t, 100000f64), t0);
            assert_eq!(roundf(xs[1].t, 100000f64), t1);
        }
    }

    #[test]
    fn test_cylinder_normal_vector() {
        let c = Cylinder::default();
        let examples = vec![
            (
                "1",
                TypedVec::point(1.0, 0.0, 0.0),
                TypedVec::vector(1.0, 0.0, 0.0),
            ),
            (
                "2",
                TypedVec::point(0.0, 5.0, -1.0),
                TypedVec::vector(0.0, 0.0, -1.0),
            ),
            (
                "3",
                TypedVec::point(0.0, -2.0, 1.0),
                TypedVec::vector(0.0, 0.0, 1.0),
            ),
            (
                "4",
                TypedVec::point(-1.0, 1.0, 0.0),
                TypedVec::vector(-1.0, 0.0, 0.0),
            ),
        ];

        for e in examples {
            let (name, p, n) = e;
            dbg!(name);
            assert_eq!(c.local_normal_at(p).unwrap(), n)
        }
    }

    #[test]
    fn test_constrained_cylinder() {
        let cyl = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            ..Default::default()
        };
        let examples = vec![
            (
                "1",
                TypedVec::point(0.0, 1.5, 0.0),
                TypedVec::vector(0.1, 1.0, 0.0),
                0,
            ),
            (
                "2",
                TypedVec::point(0.0, 3.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                "3",
                TypedVec::point(0.0, 0.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                "4",
                TypedVec::point(0.0, 2.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                "5",
                TypedVec::point(0.0, 1.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                "6",
                TypedVec::point(0.0, 1.5, -2.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                2,
            ),
        ];
        for e in examples {
            let (name, point, direction, count) = e;
            dbg!(name);
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cyl.intersect(r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn test_capped_cylinder() {
        let cyl = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };
        let examples = vec![
            (
                "1",
                TypedVec::point(0.0, 3.0, 0.0),
                TypedVec::vector(0.0, -1.0, 0.0),
                2,
            ),
            (
                "2",
                TypedVec::point(0.0, 3.0, -2.0),
                TypedVec::vector(0.0, -1.0, 2.0),
                2,
            ),
            (
                "3",
                TypedVec::point(0.0, 4.0, -2.0),
                TypedVec::vector(0.0, -1.0, 1.0),
                2,
            ),
            (
                "4",
                TypedVec::point(0.0, 0.0, -2.0),
                TypedVec::vector(0.0, 1.0, 2.0),
                2,
            ),
            (
                "5",
                TypedVec::point(0.0, -1.0, -2.0),
                TypedVec::vector(0.0, 1.0, 1.0),
                2,
            ),
        ];
        for e in examples {
            let (name, point, direction, count) = e;
            dbg!(name);
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cyl.intersect(r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn test_closed_cylinder_normal_vector() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
            ..Default::default()
        };
        let examples = vec![
            (
                "1",
                TypedVec::point(0.0, 1.0, 0.0),
                TypedVec::vector(0.0, -1.0, 0.0),
            ),
            (
                "2",
                TypedVec::point(0.5, 1.0, 0.0),
                TypedVec::vector(0.0, -1.0, 0.0),
            ),
            (
                "3",
                TypedVec::point(0.0, 1.0, 0.5),
                TypedVec::vector(0.0, -1.0, 0.0),
            ),
            (
                "4",
                TypedVec::point(-0.0, 2.0, 0.0),
                TypedVec::vector(-0.0, 1.0, 0.0),
            ),
            (
                "5",
                TypedVec::point(0.5, 2.0, 0.0),
                TypedVec::vector(0.0, 1.0, 0.0),
            ),
            (
                "6",
                TypedVec::point(0.0, 2.0, 0.5),
                TypedVec::vector(0.0, 1.0, 0.0),
            ),
        ];

        for e in examples {
            let (name, p, n) = e;
            dbg!(name);
            assert_eq!(c.local_normal_at(p).unwrap(), n)
        }
    }
}
