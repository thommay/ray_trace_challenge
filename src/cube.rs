use crate::hittable::HittableImpl;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use crate::{shape, EPSILON};
use anyhow::Result;

shape!(Cube);

impl<'a> Cube<'a> {
    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1f64 - origin;
        let tmax_numerator = 1f64 - origin;
        let (tmin, tmax) = (tmin_numerator / direction, tmax_numerator / direction);
        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = self.check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = self.check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = self.check_axis(ray.origin.z, ray.direction.z);
        let mins = vec![xtmin, ytmin, ztmin];
        let tmin = mins
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        let maxes = vec![xtmax, ytmax, ztmax];
        let tmax = maxes
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        if tmin > tmax {
            return vec![];
        }
        vec![
            Intersection::new(*tmin, self),
            Intersection::new(*tmax, self),
        ]
    }

    fn local_normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        let possibles = vec![p.x.abs(), p.y.abs(), p.z.abs()];
        let maxc = possibles
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        if (maxc - p.x.abs()).abs() < EPSILON {
            Ok(TypedVec::vector(p.x, 0f64, 0f64))
        } else if (maxc - p.y.abs()).abs() < EPSILON {
            Ok(TypedVec::vector(0f64, p.y, 0f64))
        } else {
            Ok(TypedVec::vector(0f64, 0f64, p.z))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cube::Cube;
    use crate::hittable::{Hittable, HittableImpl};
    use crate::ray::Ray;
    use crate::vec3::TypedVec;

    #[test]
    fn test_cube_ray_intersect() {
        let c = Cube::default();
        let examples = vec![
            (
                "+x",
                TypedVec::point(5f64, 0.5, 0f64),
                TypedVec::vector(-1f64, 0f64, 0f64),
                4f64,
                6f64,
            ),
            (
                "-x",
                TypedVec::point(-5f64, 0.5, 0f64),
                TypedVec::vector(1f64, 0f64, 0f64),
                4f64,
                6f64,
            ),
            (
                "+y",
                TypedVec::point(0.5, 5f64, 0f64),
                TypedVec::vector(0f64, -1f64, 0f64),
                4f64,
                6f64,
            ),
            (
                "-y",
                TypedVec::point(0.5, -5f64, 0f64),
                TypedVec::vector(0f64, 1f64, 0f64),
                4f64,
                6f64,
            ),
            (
                "+z",
                TypedVec::point(0.5, 0f64, 5f64),
                TypedVec::vector(0f64, 0f64, -1f64),
                4f64,
                6f64,
            ),
            (
                "-z",
                TypedVec::point(0.5, 0f64, -5f64),
                TypedVec::vector(0f64, 0f64, 1f64),
                4f64,
                6f64,
            ),
            (
                "+x",
                TypedVec::point(0f64, 0.5, 0f64),
                TypedVec::vector(0f64, 0f64, 1f64),
                -1f64,
                1f64,
            ),
        ];
        for test in examples {
            let (name, origin, direction, t1, t2) = test;
            dbg!(&name);
            let r = Ray::new(origin, direction);
            let xs = c.intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t1);
            assert_eq!(xs[1].t, t2);
        }
    }
    #[test]
    fn test_cube_ray_missing() {
        let c = Cube::default();
        let examples = vec![
            (
                "+x",
                TypedVec::point(-2f64, 0.0, 0f64),
                TypedVec::vector(0.2673f64, 0.5345f64, 0.8018f64),
                4f64,
                6f64,
            ),
            (
                "-x",
                TypedVec::point(0f64, -2.0, 0f64),
                TypedVec::vector(0.8018f64, 0.2673f64, 0.5345f64),
                4f64,
                6f64,
            ),
            (
                "+y",
                TypedVec::point(0.0, 0f64, -2f64),
                TypedVec::vector(0.5345f64, 0.8018f64, 0.2673f64),
                4f64,
                6f64,
            ),
            (
                "-y",
                TypedVec::point(2.0, -0f64, 2f64),
                TypedVec::vector(0f64, 0f64, -1f64),
                4f64,
                6f64,
            ),
            (
                "+z",
                TypedVec::point(0.0, 2f64, 2f64),
                TypedVec::vector(0f64, -1f64, 0f64),
                4f64,
                6f64,
            ),
            (
                "-z",
                TypedVec::point(2.0, 2f64, -0f64),
                TypedVec::vector(1f64, 0f64, 0f64),
                4f64,
                6f64,
            ),
        ];
        for test in examples {
            let (name, origin, direction, _, _) = test;
            dbg!(&name);
            let r = Ray::new(origin, direction);
            let xs = c.intersect(r);
            assert_eq!(xs.len(), 0);
        }
    }
    #[test]
    fn test_cube_local_normal() {
        let c = Cube::default();
        let examples = vec![
            (
                "+x",
                TypedVec::point(1f64, 0.5, -0.8f64),
                TypedVec::vector(1.0, 0.0, 0.0),
            ),
            (
                "-x",
                TypedVec::point(-1f64, -0.2, 0.9f64),
                TypedVec::vector(-1.0, 0.0, 0.0),
            ),
            (
                "+y",
                TypedVec::point(-0.4, 1f64, -0.1f64),
                TypedVec::vector(0.0, 1.0, 0.0),
            ),
            (
                "-y",
                TypedVec::point(0.3, -1f64, -0.7f64),
                TypedVec::vector(0f64, -1f64, 0f64),
            ),
            (
                "+z",
                TypedVec::point(-0.6, 0.3f64, 1f64),
                TypedVec::vector(0f64, -0f64, 1f64),
            ),
            (
                "-z",
                TypedVec::point(0.4, 0.4f64, -1f64),
                TypedVec::vector(0f64, 0f64, -1f64),
            ),
            (
                "1s",
                TypedVec::point(1f64, 1f64, 1f64),
                TypedVec::vector(1f64, 0f64, -0f64),
            ),
            (
                "-1s",
                TypedVec::point(-1f64, -1f64, -1f64),
                TypedVec::vector(-1f64, 0f64, -0f64),
            ),
        ];
        for test in examples {
            let (name, point, normal) = test;
            dbg!(&name);
            assert_eq!(c.normal_at(point).unwrap(), normal);
        }
    }
}
