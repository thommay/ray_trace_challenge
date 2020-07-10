use crate::cylinder::Capped;
use crate::hittable::HittableImpl;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use crate::{shape, ZeroIsh, EPSILON};
use anyhow::Result;
use std::f64::INFINITY;

shape!(Cone, nodefault, minimum -> f64, maximum -> f64, closed -> bool);

impl Default for Cone {
    fn default() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            transform: None,
            parent: None,
            material: Material::default(),
            closed: false,
        }
    }
}

impl Cone {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
            + 2.0 * ray.origin.z * ray.direction.z;

        if a.zeroish() && b.zeroish() {
            return self.intersect_caps(ray);
        }

        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);
        let mut xs = vec![];
        if a.zeroish() {
            let t = -c / (2.0 * b);
            xs.push(Intersection::new(t, self));
        } else {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return vec![];
            }
            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);
            let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };
            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, self));
            }
            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, self));
            }
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
            let mut y = (p.x.powi(2) + p.z.powi(2)).sqrt();
            if p.y > 0.0 {
                y = -y;
            }
            Ok(TypedVec::vector(p.x, y, p.z))
        }
    }
}

impl Capped for Cone {
    fn closed(&self) -> bool {
        self.closed
    }

    fn minimum(&self) -> f64 {
        self.minimum
    }

    fn maximum(&self) -> f64 {
        self.maximum
    }

    fn check_caps(&self, ray: Ray, t: f64, y: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x.powi(2) + z.powi(2)) <= y.abs()
    }
}

impl HittableImpl for Cone {
    fn h_intersect(&self, ray: Ray) -> Vec<Intersection> {
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
    use crate::cone::Cone;
    use crate::hittable::Hittable;
    use crate::ray::Ray;
    use crate::roundf;
    use crate::vec3::TypedVec;

    #[test]
    fn test_cone_intersect() {
        let c = Cone::default();
        let examples = vec![
            (
                "1",
                TypedVec::point(0.0, 0.0, -5.0),
                TypedVec::vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                "2",
                TypedVec::point(0.0, 0.0, -5.0),
                TypedVec::vector(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                "3",
                TypedVec::point(1.0, 1.0, -5.0),
                TypedVec::vector(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];
        for e in examples {
            let (name, o, d, t0, t1) = e;
            dbg!(&name);
            let direction = d.normalize();
            let r = Ray::new(o, direction);
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(roundf(xs[0].t, 100000f64), t0);
            assert_eq!(roundf(xs[1].t, 100000f64), t1);
        }
    }

    #[test]
    fn test_cone_parallel_half() {
        let c = Cone::default();
        let direction = TypedVec::vector(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(TypedVec::point(0.0, 0.0, -1.0), direction);
        let xs = c.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(roundf(xs[0].t, 100000f64), 0.35355);
    }

    #[test]
    fn test_capped_cone() {
        let c = Cone {
            minimum: -0.5,
            maximum: 0.5,
            closed: true,
            ..Default::default()
        };
        let examples = vec![
            (
                "1",
                TypedVec::point(0.0, 0.0, -5.0),
                TypedVec::vector(0.0, 1.0, 0.0),
                0,
            ),
            (
                "2",
                TypedVec::point(0.0, 0.0, -0.25),
                TypedVec::vector(0.0, 1.0, 1.0),
                2,
            ),
            (
                "3",
                TypedVec::point(0.0, 0.0, -0.25),
                TypedVec::vector(0.0, 1.0, 0.0),
                4,
            ),
        ];
        for e in examples {
            let (name, point, direction, count) = e;
            dbg!(name);
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = c.intersect(r);
            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn test_cone_normal_vector() {
        let c = Cone::default();
        let examples = vec![
            (
                "1",
                TypedVec::point(0.0, 0.0, 0.0),
                TypedVec::vector(0.0, 0.0, 0.0),
            ),
            (
                "2",
                TypedVec::point(1.0, 1.0, 1.0),
                TypedVec::vector(1.0, -2f64.sqrt(), 1.0),
            ),
            (
                "3",
                TypedVec::point(-1.0, -1.0, 0.0),
                TypedVec::vector(-1.0, 1.0, 0.0),
            ),
        ];

        for e in examples {
            let (name, p, n) = e;
            dbg!(name);
            assert_eq!(c.local_normal_at(p).unwrap(), n)
        }
    }
}
