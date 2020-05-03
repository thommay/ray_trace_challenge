use crate::ray::Ray;
use crate::sphere::Hittable;
use crate::vec3::TypedVec;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Index;

const EPSILON: f64 = 0.0001;
#[derive(Clone, Debug, PartialEq)]
pub struct PreComp<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    pub(crate) eyev: TypedVec,
    inside: bool,
    pub(crate) normalv: TypedVec,
    pub(crate) obj: &'a H,
    pub(crate) point: TypedVec,
    pub(crate) over_point: TypedVec,
    t: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    pub t: f64,
    pub obj: &'a H,
}

impl<'a, H> Intersection<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    pub fn new(t: f64, obj: &'a H) -> Self {
        Intersection { t, obj }
    }
    pub fn precompute(&self, ray: Ray) -> PreComp<H> {
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
        PreComp {
            t: self.t,
            obj: self.obj,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

impl<'a, H> PartialOrd for Intersection<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

#[derive(Debug)]
pub struct Intersections<'a, H>(Vec<Intersection<'a, H>>)
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug;

impl<'a, H> Intersections<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    pub fn new() -> Self {
        Intersections(Vec::new())
    }

    pub fn from_iter<T: IntoIterator<Item = Intersection<'a, H>>>(iter: T) -> Self {
        let mut c = Self::new();
        for i in iter {
            c.push(i);
        }
        c
    }

    pub fn push(&mut self, elem: Intersection<'a, H>) {
        self.0.push(elem);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&mut self) -> Option<&Intersection<H>> {
        self.0.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.0.iter().filter(|&x| x.t > 0f64).take(1).next()
    }
}

impl<'a, H> IntoIterator for Intersections<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    type Item = Intersection<'a, H>;
    type IntoIter = IntersectionsIterator<'a, H>;

    fn into_iter(self) -> Self::IntoIter {
        IntersectionsIterator { i: self, pos: 0 }
    }
}

pub struct IntersectionsIterator<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    i: Intersections<'a, H>,
    pos: usize,
}

impl<'a, H> Iterator for IntersectionsIterator<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    type Item = Intersection<'a, H>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if self.pos < self.i.0.len() {
            Some(self.i[self.pos].clone())
        } else {
            None
        }
    }
}

impl<'a, H> Index<usize> for Intersections<'a, H>
where
    H: Hittable + PartialEq + PartialOrd + Clone + Debug,
{
    type Output = Intersection<'a, H>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::{Intersection, Intersections};
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vec3::TypedVec;
    use std::f64::EPSILON;

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
        let comps = i.precompute(r);
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
        let comps = i.precompute(r);
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
        let comps = i.precompute(r);
        assert!(comps.over_point.z < -EPSILON / 2f64);
        assert!(comps.point.z > comps.over_point.z)
    }
}
