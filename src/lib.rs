pub mod camera;
pub mod canvas;
pub mod colour;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod hittable;
pub mod intersection;
pub mod lighting;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod plane;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod world;

pub const EPSILON: f64 = 0.0001;
trait ZeroIsh {
    fn zeroish(&self) -> bool;
}

impl ZeroIsh for f64 {
    fn zeroish(&self) -> bool {
        self.abs() <= EPSILON
    }
}

#[macro_export]
macro_rules! shape {
    (@real$name:ident, $($derive:meta):*, $($n:tt -> $t:ty)*) => {
        use crate::group::Group;
        // use ray_trace_challenge_derive::Groupable;
        use crate::group::Groupable;
        use std::cell::RefCell;
        use std::rc::Rc;
        #[derive($($derive,)*)]
        pub struct $name<'a> {
            pub material: Material,
            pub parent: Option<Rc<RefCell<Group<'a>>>>,
            pub transform: Option<Matrix<f64>>,
            $(
            pub $n: $t,
            )*
        }

        impl<'a> Groupable<'a> for $name<'a> {
            fn set_parent(&mut self, parent: &Rc<RefCell<Group<'a>>>) {
                let parent = Rc::clone(parent);
                self.parent = Some(parent);
            }
        }

        impl<'a> HittableImpl for $name<'a> {
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
    };
    ($name:ident, nodefault, $($n:tt -> $t:ty),*) => {
        shape!(@real $name, Clone:Debug:PartialOrd:PartialEq, $($n -> $t)*);
    };
    ($name:ident, $($n:tt -> $t:ty),*) => {
        shape!(@real $name, Clone:Debug:Default:PartialOrd:PartialEq, $($n -> $t)*);
    };
    ($name:ident) => {
        shape!(@real $name, Clone:Debug:Default:PartialOrd:PartialEq,);
    };
}

#[macro_export]
macro_rules! group {
    ($parent:ident, $child:ident) => {
        $parent.borrow_mut().set_child(&$child);
        $child.set_parent(&$parent);
    };
}

#[cfg(test)]
pub fn roundf(val: f64, factor: f64) -> f64 {
    (val * factor).round() / factor
}
