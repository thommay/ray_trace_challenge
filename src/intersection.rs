use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use crate::EPSILON;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub struct PreComp<'a> {
    pub(crate) eyev: TypedVec,
    inside: bool,
    pub(crate) normalv: TypedVec,
    pub(crate) obj: &'a dyn Hittable,
    pub(crate) point: TypedVec,
    pub(crate) over_point: TypedVec,
    pub(crate) under_point: TypedVec,
    pub(crate) reflectv: TypedVec,
    t: f64,
    pub(crate) n1: f64,
    pub(crate) n2: f64,
}

impl<'a> PreComp<'a> {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot_product(self.normalv);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1f64 - cos.powi(2));
            if sin2_t > 1f64 {
                return 1.0;
            }
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1f64 - r0) * (1f64 - cos).powi(5)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub obj: &'a dyn Hittable,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, obj: &'a dyn Hittable) -> Self {
        Intersection { t, obj }
    }
    pub fn precompute(&self, ray: Ray, xs: &Intersections) -> PreComp {
        let point = ray.position(self.t);
        let mut normalv = self.obj.normal_at(point).unwrap();
        let eyev = -ray.direction;
        let inside = if normalv.dot_product(eyev) < 0.0 {
            normalv = -normalv;
            true
        } else {
            false
        };
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = ray.direction.reflect(normalv);
        let mut containers: Vec<&dyn Hittable> = vec![];
        let mut n1 = 0f64;
        let mut n2 = 0f64;
        for i in xs.clone().into_iter() {
            if self == &i {
                n1 = if containers.is_empty() {
                    1f64
                } else {
                    containers.last().unwrap().material().refractive_index
                };
            }
            if containers.contains(&i.obj) {
                containers.retain(|&x| x != i.obj);
            } else {
                containers.push(i.obj)
            }
            if self == &i {
                n2 = if containers.is_empty() {
                    1f64
                } else {
                    containers.last().unwrap().material().refractive_index
                };
                break;
            }
        }
        PreComp {
            t: self.t,
            obj: self.obj,
            point,
            eyev,
            normalv,
            inside,
            over_point,
            under_point,
            reflectv,
            n1,
            n2,
        }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Intersections<'a>(Vec<Intersection<'a>>);

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Intersections(Vec::new())
    }

    pub fn from_iter<T: IntoIterator<Item = Intersection<'a>>>(iter: T) -> Self {
        let mut c = Self::new();
        for i in iter {
            c.push(i);
        }
        c
    }

    pub fn push(&mut self, elem: Intersection<'a>) {
        self.0.push(elem);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn hit(&mut self) -> Option<&Intersection> {
        self.0.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.0.iter().filter(|&x| x.t > 0f64).take(1).next()
    }
}

impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = IntersectionsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IntersectionsIterator { i: self, pos: 0 }
    }
}

pub struct IntersectionsIterator<'a> {
    i: Intersections<'a>,
    pos: usize,
}

impl<'a> Iterator for IntersectionsIterator<'a> {
    type Item = Intersection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        self.pos += 1;
        if pos < self.i.0.len() {
            Some(self.i[pos].clone())
        } else {
            None
        }
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection;
    use crate::intersection::{Intersection, Intersections};
    use crate::matrix::Matrix;
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vec3::TypedVec;
    use crate::EPSILON;

    fn roundf(val: f64, factor: f64) -> f64 {
        (val * factor).round() / factor
    }

    #[test]
    fn test_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::from_iter(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], i1);
        assert_eq!(xs[1], i2);
    }

    #[test]
    fn test_hit_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = Intersections::from_iter(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.hit(), Some(&i1))
    }

    #[test]
    fn test_hit_mixed() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = Intersections::from_iter(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.hit(), Some(&i2))
    }

    #[test]
    fn test_hit_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let mut xs = Intersections::from_iter(vec![i1.clone(), i2.clone()]);
        assert_eq!(xs.hit(), None)
    }

    #[test]
    fn test_hit_lowest() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let mut xs = Intersections::from_iter(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);
        assert_eq!(xs.hit(), Some(&i4))
    }

    #[test]
    fn test_precompute() {
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.obj, i.obj);
        assert_eq!(comps.point, TypedVec::point(0f64, 0f64, -1f64));
        assert_eq!(comps.eyev, TypedVec::vector(0f64, 0f64, -1f64));
        assert_eq!(comps.normalv, TypedVec::vector(0f64, 0f64, -1f64));
        assert!(!comps.inside)
    }

    #[test]
    fn test_precompute_inside() {
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let s = Sphere::new();
        let i = Intersection::new(1.0, &s);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.obj, i.obj);
        assert_eq!(comps.point, TypedVec::point(0f64, 0f64, 1f64));
        assert_eq!(comps.eyev, TypedVec::vector(0f64, 0f64, -1f64));
        assert_eq!(comps.normalv, TypedVec::vector(0f64, 0f64, -1f64));
        assert!(comps.inside)
    }

    #[test]
    fn test_hit_offset_point() {
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let mut s = Sphere::new();
        s.transform = Option::from(Matrix::translation(0f64, 0f64, 1f64));
        let i = Intersection::new(5.0, &s);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert!(comps.over_point.z < -EPSILON / 2f64);
        assert!(comps.point.z > comps.over_point.z)
    }

    #[test]
    fn test_precompute_reflection() {
        let s = Plane::default();
        let r = Ray::new(
            TypedVec::point(0f64, 1f64, -1f64),
            TypedVec::vector(0f64, -2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64),
        );
        let i = Intersection::new(2f64.sqrt(), &s);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(
            comps.reflectv,
            TypedVec::vector(0f64, 2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64)
        )
    }

    #[test]
    fn test_find_nx_at_intersections() {
        let mut a = Sphere::glass();
        a.transform = Some(Matrix::scaling(2f64, 2f64, 2f64));
        a.material.refractive_index = 1.5;
        let mut b = Sphere::glass();
        b.transform = Some(Matrix::translation(0f64, 0f64, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = Sphere::glass();
        c.transform = Some(Matrix::translation(0f64, 0f64, 0.25));
        c.material.refractive_index = 2.5;
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -4f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let x = vec![
            Intersection::new(2f64, &a),
            Intersection::new(2.75f64, &b),
            Intersection::new(3.25f64, &c),
            Intersection::new(4.75f64, &b),
            Intersection::new(5.25f64, &c),
            Intersection::new(6f64, &a),
        ];
        let xs = Intersections::from_iter(x);
        for test in vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ]
        .iter()
        {
            let (i, n1, n2) = test;
            let c = xs[*i].precompute(r, &xs);
            assert_eq!(c.n1, *n1);
            assert_eq!(c.n2, *n2);
        }
    }

    #[test]
    fn test_under_point() {
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let mut c = Sphere::glass();
        c.transform = Some(Matrix::translation(0f64, 0f64, 1f64));
        let i = Intersection::new(5f64, &c);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert!(comps.under_point.z > intersection::EPSILON / 2f64);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn test_schlick_total_internal() {
        let s = Sphere::glass();
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -2f64.sqrt() / 2f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        let xs = Intersections::from_iter(vec![
            Intersection::new(-2f64.sqrt() / 2f64, &s),
            Intersection::new(2f64.sqrt() / 2f64, &s),
        ]);
        let comps = xs[1].precompute(r, &xs);
        assert_eq!(comps.schlick(), 1f64)
    }

    #[test]
    fn test_schlick_perpendicular() {
        let s = Sphere::glass();
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        let xs = Intersections::from_iter(vec![
            Intersection::new(-1f64, &s),
            Intersection::new(1f64, &s),
        ]);
        let comps = xs[1].precompute(r, &xs);
        assert_eq!(roundf(comps.schlick(), 100_000f64), 0.04)
    }

    #[test]
    fn test_schlick_n2_gt_n1() {
        let s = Sphere::glass();
        let r = Ray::new(
            TypedVec::point(0f64, 0.99f64, -2f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let xs = Intersections::from_iter(vec![Intersection::new(1.8589f64, &s)]);
        let comps = xs[0].precompute(r, &xs);
        assert_eq!(roundf(comps.schlick(), 100_000f64), 0.48873)
    }
}
