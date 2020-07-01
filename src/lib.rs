pub mod camera;
pub mod canvas;
pub mod colour;
pub mod cone;
pub mod cube;
pub mod cylinder;
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

#[cfg(test)]
pub fn roundf(val: f64, factor: f64) -> f64 {
    (val * factor).round() / factor
}
