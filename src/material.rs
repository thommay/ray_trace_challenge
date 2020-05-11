use crate::colour::{Colour, BLACK, WHITE};
use crate::lighting::Point;
use crate::pattern::Pattern;
use crate::sphere::HittableImpl;
use crate::vec3::TypedVec;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Material {
    pub ambient: f64,
    pub colour: Colour,
    pub diffuse: f64,
    pub shininess: f64,
    pub specular: f64,
    pub pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            colour: *WHITE,
            diffuse: 0.9,
            shininess: 200f64,
            specular: 0.9,
            pattern: None,
        }
    }
}

impl Material {
    pub fn new(
        ambient: f64,
        colour: Colour,
        diffuse: f64,
        shininess: f64,
        specular: f64,
        pattern: Option<Pattern>,
    ) -> Self {
        Material {
            ambient,
            colour,
            diffuse,
            shininess,
            specular,
            pattern,
        }
    }

    pub fn lighting<'l>(
        &self,
        object: &'l dyn HittableImpl,
        light: Point,
        point: TypedVec,
        eyev: TypedVec,
        normalv: TypedVec,
        in_shadow: bool,
    ) -> Colour {
        let colour = if let Some(pattern) = &self.pattern {
            object.pattern_at(pattern, point).unwrap()
        } else {
            self.colour
        } * light.intensity;

        let lightv = (light.position - point).normalize();
        let ambient = colour * self.ambient;

        if in_shadow {
            return ambient;
        }
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
    use crate::colour::*;
    use crate::lighting;
    use crate::material::Material;
    use crate::pattern::Pattern;
    use crate::pattern::PatternType::Stripe;
    use crate::sphere::Sphere;
    use crate::vec3::TypedVec;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref M: Material = Material::default();
        static ref S: Sphere = Sphere::default();
        static ref POSITION: TypedVec = TypedVec::point(0f64, 0f64, 0f64);
    }

    #[test]
    fn test_light_behind_eye() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE);
        let r = M.lighting(&*S, l, *POSITION, eyev, normalv, false);
        assert_eq!(r, Colour::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn test_light_behind_eye_off_45() {
        let eyev = TypedVec::vector(0f64, 2f64.sqrt() / 2f64, -2f64.sqrt() / 2f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE);
        let r = M.lighting(&*S, l, *POSITION, eyev, normalv, false);
        assert_eq!(r, Colour::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn test_light_eye_opp_surface_off_45() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 10f64, -10f64), *WHITE);
        let r = M.lighting(&*S, l, *POSITION, eyev, normalv, false);
        assert_eq!(r.round(10000f64), Colour::new(0.7364, 0.7364, 0.7364))
    }

    #[test]
    fn test_light_reflection_vec() {
        let eyev = TypedVec::vector(0f64, -2f64.sqrt() / 2f64, -2f64.sqrt() / 2f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 10f64, -10f64), *WHITE);
        let r = M.lighting(&*S, l, *POSITION, eyev, normalv, false);
        assert_eq!(r.round(10000f64), Colour::new(1.6364, 1.6364, 1.6364))
    }

    #[test]
    fn test_light_behind_surface() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, 10f64), *WHITE);
        let r = M.lighting(&*S, l, *POSITION, eyev, normalv, false);
        assert_eq!(r.round(100f64), Colour::new(0.1, 0.1, 0.1))
    }

    #[test]
    fn test_surface_in_shadow() {
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE);
        let r = M.lighting(&*S, l, *POSITION, eyev, normalv, true);
        assert_eq!(r.round(100f64), Colour::new(0.1, 0.1, 0.1))
    }

    #[test]
    fn test_material_with_pattern() {
        let p = Pattern::new(Stripe, *WHITE, *BLACK, false);
        let m = Material::new(1f64, *WHITE, 0f64, 200f64, 0f64, Some(p));
        let eyev = TypedVec::vector(0f64, 0f64, -1f64);
        let normalv = TypedVec::vector(0f64, 0f64, -1f64);
        let l = lighting::Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE);
        let c1 = m.lighting(
            &*S,
            l,
            TypedVec::point(0.9, 0f64, 0f64),
            eyev,
            normalv,
            true,
        );
        let c2 = m.lighting(
            &*S,
            l,
            TypedVec::point(1.1, 0f64, 0f64),
            eyev,
            normalv,
            true,
        );
        assert_eq!(c1, *WHITE);
        assert_eq!(c2, *BLACK);
    }
}
