use crate::hittable::{Hittable, HittableImpl};
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Group {
    pub transform: Option<Matrix<f64>>,
    pub material: Material,
    parent: Option<Rc<RefCell<Group>>>,
    children: Vec<&'static dyn Hittable>,
}

impl Group {
    pub fn set_parent(&mut self, parent: &Rc<RefCell<Group>>) {
        let parent = Rc::clone(parent);
        self.parent = Some(parent);
    }

    pub fn set_child<T>(&mut self, child: &'static T)
    where
        T: Hittable,
    {
        self.children.push(child);
    }
}

impl HittableImpl for Group {
    fn h_intersect(&self, ray: Ray) -> Vec<Intersection> {
        unimplemented!()
    }

    fn normal_at(&self, p: TypedVec) -> Result<TypedVec> {
        unimplemented!()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Option<Matrix<f64>> {
        &self.transform
    }
}
