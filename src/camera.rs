use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::TypedVec;
use crate::world::World;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Camera {
    hsize: f64,
    vsize: f64,
    fov: f64,
    aspect: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    pub transform: Matrix<f64>,
}

impl Camera {
    pub fn new(hsize: f64, vsize: f64, fov: f64) -> Self {
        let aspect = hsize / vsize;
        let half_view = (fov / 2f64).tan();
        let (half_width, half_height) = if aspect >= 1f64 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2f64) / hsize;
        Camera {
            hsize,
            vsize,
            fov,
            aspect,
            half_width,
            half_height,
            pixel_size,
            ..Default::default()
        }
    }
    fn ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let xoffset = (x + 0.5) * self.pixel_size;
        let yoffset = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform.inverse().unwrap() * TypedVec::point(world_x, world_y, -1f64);
        let origin = self.transform.inverse().unwrap() * TypedVec::point(0f64, 0f64, 0f64);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World<Sphere>) -> Canvas {
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
        for y in 0..self.vsize as usize {
            for x in 0..self.hsize as usize {
                let ray = self.ray_for_pixel(x as f64, y as f64);
                let colour = world.colour_at(ray);
                image.write_pixel(x, y, colour);
            }
        }
        image
    }
}

impl Default for Camera {
    fn default() -> Self {
        let fov = PI / 2f64;
        let half_view = (fov / 2f64).tan();
        Self {
            hsize: 100f64,
            vsize: 100f64,
            fov,
            half_height: half_view,
            half_width: half_view,
            aspect: 1f64,
            pixel_size: (half_view * 2f64) / 100f64,
            transform: Matrix::identity(4),
        }
    }
}

pub fn view_transform(from: TypedVec, to: TypedVec, up: TypedVec) -> Matrix<f64> {
    let forward = (to - from).normalize();
    let left = forward.cross_product(up.normalize());
    let true_up = left.cross_product(forward);
    let orientation = Matrix::from_iter(
        4,
        4,
        vec![
            left.x, left.y, left.z, 0f64, true_up.x, true_up.y, true_up.z, 0f64, -forward.x,
            -forward.y, -forward.z, 0f64, 0f64, 0f64, 0f64, 1f64,
        ],
    );
    orientation * Matrix::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod test {
    use crate::camera::{view_transform, Camera};
    use crate::colour::Colour;
    use crate::matrix::{Axis, Matrix};
    use crate::vec3::TypedVec;
    use crate::world;
    use std::f64::consts::PI;

    #[test]
    fn test_default() {
        let from = TypedVec::point(0f64, 0f64, 0f64);
        let to = TypedVec::point(0f64, 0f64, -1f64);
        let up = TypedVec::vector(0f64, 1f64, 0f64);
        assert_eq!(view_transform(from, to, up), Matrix::identity(4))
    }

    #[test]
    fn test_view_positive_z() {
        let from = TypedVec::point(0f64, 0f64, 0f64);
        let to = TypedVec::point(0f64, 0f64, 1f64);
        let up = TypedVec::vector(0f64, 1f64, 0f64);
        assert_eq!(
            view_transform(from, to, up),
            Matrix::scaling(-1f64, 1f64, -1f64)
        )
    }

    #[test]
    fn test_view_moves_the_world() {
        let from = TypedVec::point(0f64, 0f64, 8f64);
        let to = TypedVec::point(0f64, 0f64, 0f64);
        let up = TypedVec::vector(0f64, 1f64, 0f64);
        assert_eq!(
            view_transform(from, to, up),
            Matrix::translation(0f64, 0f64, -8f64)
        )
    }

    #[test]
    fn test_arbitrary_view_transform() {
        let from = TypedVec::point(1f64, 3f64, 2f64);
        let to = TypedVec::point(4f64, -2f64, 8f64);
        let up = TypedVec::vector(1f64, 1f64, 0f64);
        assert_eq!(
            view_transform(from, to, up).round(100000f64),
            Matrix::from_iter(
                4,
                4,
                vec![
                    -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843,
                    -0.35857, 0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000
                ]
            )
        )
    }

    #[test]
    fn test_pixel_size_vert() {
        let c = Camera::new(125f64, 200f64, PI / 2f64);
        assert_eq!(round(c.pixel_size, 10000f64), 0.01)
    }
    #[test]
    fn test_pixel_size_horiz() {
        let c = Camera::new(200f64, 125f64, PI / 2f64);
        assert_eq!(round(c.pixel_size, 10000f64), 0.01)
    }
    fn round(val: f64, scale: f64) -> f64 {
        (val * scale).round() / scale
    }

    #[test]
    fn test_ray_through_centre() {
        let c = Camera::new(201f64, 101f64, PI / 2f64);
        let r = c.ray_for_pixel(100f64, 50f64);
        assert_eq!(r.origin, TypedVec::point(0f64, 0f64, 0f64));
        assert_eq!(
            r.direction.round(10f64),
            TypedVec::vector(0f64, 0f64, -1f64)
        );
    }

    #[test]
    fn test_ray_through_corner() {
        let c = Camera::new(201f64, 101f64, PI / 2f64);
        let r = c.ray_for_pixel(0f64, 0f64);
        assert_eq!(r.origin, TypedVec::point(0f64, 0f64, 0f64));
        assert_eq!(
            r.direction.round(100000f64),
            TypedVec::vector(0.66519f64, 0.33259f64, -0.66851f64)
        );
    }

    #[test]
    fn test_ray_with_transformed_camera() {
        let mut c = Camera::new(201f64, 101f64, PI / 2f64);
        c.transform = Matrix::rotation(Axis::Y, PI / 4f64) * Matrix::translation(0f64, -2f64, 5f64);
        let r = c.ray_for_pixel(100f64, 50f64);
        assert_eq!(r.origin, TypedVec::point(0f64, 2f64, -5f64));
        assert_eq!(
            r.direction.round(10000f64),
            TypedVec::vector(2f64.sqrt() / 2f64, 0f64, -2f64.sqrt() / 2f64).round(10000f64)
        );
    }

    #[test]
    fn render_a_world() {
        let w = world::test::default_world();
        let mut c = Camera::new(11f64, 11f64, PI / 2f64);
        c.transform = view_transform(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::point(0f64, 0f64, 0f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        let image = c.render(w);
        assert_eq!(
            image.get(5, 5).unwrap().round(100000f64),
            Colour::new(0.38066, 0.47583, 0.2855)
        )
    }
}
