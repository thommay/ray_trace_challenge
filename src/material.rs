use crate::colour::{Colour, BLACK, WHITE};
use crate::lighting::Point;
use crate::vec3::TypedVec;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Material {
    pub ambient: f64,
    pub colour: Colour,
    pub diffuse: f64,
    pub shininess: f64,
    pub specular: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            colour: *WHITE,
            diffuse: 0.9,
            shininess: 200f64,
            specular: 0.9,
        }
    }
}

impl Material {
    pub fn new(ambient: f64, colour: Colour, diffuse: f64, shininess: f64, specular: f64) -> Self {
        Material {
            ambient,
            colour,
            diffuse,
            shininess,
            specular,
        }
    }

    pub fn lighting(
        &self,
        light: Point,
        point: TypedVec,
        eyev: TypedVec,
        normalv: TypedVec,
    ) -> Colour {
        let colour = self.colour * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = colour * self.ambient;

        let light_dot_normal = lightv.dot_product(normalv);
        let (diffuse, specular) = if light_dot_normal < 0f64 {
            (*BLACK, *BLACK)
        } else {
            let diffuse = colour * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot_product(eyev);
            let specular = if reflect_dot_eye <= 0.0 {
                *BLACK
            } else {
                let f = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * f
            };
            (diffuse, specular)
        };
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use crate::colour::{Colour, WHITE};
    use crate::lighting;
    use crate::material::Material;
    use crate::vec3::TypedVec;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref M: Material = Material::default();
        static ref POSITION: TypedVec = TypedVec::point(0f64, 0f64, 0f64);
    }

    #[test]
    fn test_light_behind_eye() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE);
        let r = M.lighting(l, *POSITION, eyev, normalv);
        assert_eq!(r, Colour::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn test_light_behind_eye_off_45() {
        let eyev = TypedVec::vector(0f64, 2f64.sqrt() / 2f64, -2f64.sqrt() / 2f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE);
        let r = M.lighting(l, *POSITION, eyev, normalv);
        assert_eq!(r, Colour::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn test_light_eye_opp_surface_off_45() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 10f64, -10f64), *WHITE);
        let r = M.lighting(l, *POSITION, eyev, normalv);
        assert_eq!(r.round(10000f64), Colour::new(0.7364, 0.7364, 0.7364))
    }

    #[test]
    fn test_light_reflection_vec() {
        let eyev = TypedVec::vector(0f64, -2f64.sqrt() / 2f64, -2f64.sqrt() / 2f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 10f64, -10f64), *WHITE);
        let r = M.lighting(l, *POSITION, eyev, normalv);
        assert_eq!(r.round(10000f64), Colour::new(1.6364, 1.6364, 1.6364))
    }

    #[test]
    fn test_light_behind_surface() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, 10f64), *WHITE);
        let r = M.lighting(l, *POSITION, eyev, normalv);
        assert_eq!(r.round(100f64), Colour::new(0.1, 0.1, 0.1))
    }
}
