use crate::hittable::{Hittable, HittableImpl};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use anyhow::Result;
use ray_trace_challenge_derive::Groupable;
use std::cell::RefCell;
use std::rc::Rc;

mod tree;

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Groupable)]
pub struct Group<'a> {
    pub transform: Option<Matrix<f64>>,
    pub material: Material,
    pub parent: Option<Rc<RefCell<Group<'a>>>>,
    pub children: Vec<&'a dyn Hittable>,
}

pub trait Groupable<'a> {
    fn set_parent(&mut self, parent: &Rc<RefCell<Group<'a>>>);
}

impl<'a> Group<'a> {
    pub fn set_child<T>(&'a mut self, child: &'a T)
    where
        T: Hittable,
    {
        self.children.push(child);
    }

    fn local_normal_at(&self, _: TypedVec) -> Result<TypedVec> {
        unreachable!()
    }
}

impl<'a> HittableImpl for Group<'a> {
    fn h_intersect(&self, _ray: Ray) -> Vec<Intersection> {
        unimplemented!()
    }

    fn normal_at(&self, _p: TypedVec) -> Result<TypedVec> {
        unimplemented!()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::group;
    use crate::sphere::Sphere;
    use std::cell::RefMut;

    #[test]
    fn test_add_child() {
        let g = Rc::new(RefCell::new(Group::default()));
        let mut s = Sphere::default();
        // s.set_parent(&g);
        // group!(p, s);
        {
            let mut mp: RefMut<_> = g.borrow_mut();
            mp.set_child(&s);
        }
        s.set_parent(&g);
        // assert_eq!(s.parent, Some(g))
    }
}
