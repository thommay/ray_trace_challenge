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
        use std::cell::RefCell;
        use std::rc::Rc;
        #[derive($($derive,)*)]
        pub struct $name {
            pub material: Material,
            pub parent: Option<Rc<RefCell<Group>>>,
            pub transform: Option<Matrix<f64>>,
            $(
            pub $n: $t,
            )*
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

#[cfg(test)]
pub fn roundf(val: f64, factor: f64) -> f64 {
    (val * factor).round() / factor
}
