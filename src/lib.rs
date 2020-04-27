#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod canvas;
pub mod intersection;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod vec3;
