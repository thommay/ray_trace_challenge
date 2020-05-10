use crate::colour::Colour;
use crate::intersection::{Intersection, Intersections, PreComp};
use crate::lighting::Point;
use crate::ray::Ray;
use crate::sphere::HittableImpl;
use crate::vec3::TypedVec;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct World<'a> {
    light: Point,
    pub objects: Vec<&'a dyn HittableImpl>,
}

impl<'a> Default for World<'a> {
    fn default() -> Self {
        Self {
            light: Point::new(
                TypedVec::point(-10f64, 10f64, -10f64),
                Colour::new(1f64, 1f64, 1f64),
            ),
            objects: Vec::new(),
        }
    }
}

impl<'a> World<'a> {
    pub fn new(light: Point) -> Self {
        World {
            light,
            objects: Vec::new(),
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut r: Vec<Intersection> = self.objects.iter().flat_map(|o| o.intersect(ray)).collect();
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        r
    }

    fn shade_hit(&self, comps: PreComp) -> Colour {
        let shadowed = self.is_shadowed(comps.over_point);
        comps.obj.material().lighting(
            comps.obj,
            self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        )
    }

    fn is_shadowed(&self, point: TypedVec) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let toward = v.normalize();
        let r = Ray::new(point, toward);
        if let Some(hit) = Intersections::from_iter(self.intersect(r)).hit() {
            if hit.t < distance {
                return true;
            }
        }
        false
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        Intersections::from_iter(self.intersect(ray))
            .hit()
            .map_or_else(
                || *crate::colour::BLACK,
                |x| self.shade_hit(x.precompute(ray)),
            )
    }
}

#[cfg(test)]
pub mod test {
    use crate::colour::{Colour, WHITE};
    use crate::intersection::{Intersection, Intersections};
    use crate::lighting;
    use crate::lighting::Point;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vec3::TypedVec;
    use crate::world::World;

    #[macro_export]
    macro_rules! default_world {
        ($v:ident,$s:ident,$x:ident) => {
            use crate::material::Material;
            use crate::sphere::Sphere;
            use crate::world::World;
            let $s = Sphere {
                material: Material {
                    colour: Colour::new(0.8, 1.0, 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Default::default()
                },
                ..Default::default()
            };
            let $x = Sphere {
                transform: Some(Matrix::scaling(0.5, 0.5, 0.5)),
                ..Default::default()
            };

            let mut $v = World::default();
            $v.objects.append(&mut vec![&$s, &$x]);
        };
    }
    #[test]
    fn test_world_intersect() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let xs = Intersections::from_iter(w.intersect(r));
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4f64);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6f64);
    }

    #[test]
    fn test_shading_inside() {
        default_world!(w, s1, s2);
        w.light = lighting::Point::new(TypedVec::point(0f64, 0.25, 0f64), *WHITE);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -0f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.precompute(r);
        let c = w.shade_hit(comps);
        assert_eq!(c.round(100000f64), Colour::new(0.90495, 0.90495, 0.90495));
    }

    #[test]
    fn test_shading() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let shape = w.objects[0];
        let i = Intersection::new(4f64, shape);
        let comps = i.precompute(r);
        let c = w.shade_hit(comps);
        assert_eq!(c.round(100000f64), Colour::new(0.38066, 0.47582, 0.28549));
    }

    #[test]
    fn test_miss() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        assert_eq!(w.colour_at(r), Colour::new(0f64, 0f64, 0f64))
    }

    #[test]
    fn test_hit() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        assert_eq!(
            w.colour_at(r).round(100000f64),
            Colour::new(0.38066, 0.47582, 0.28549)
        )
    }

    #[test]
    fn test_hit_behind() {
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1f64,
                ..Default::default()
            },
            ..Default::default()
        };
        let s2 = Sphere {
            transform: Some(Matrix::scaling(0.5, 0.5, 0.5)),
            material: Material {
                ambient: 1f64,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut w = World::default();
        w.objects.append(&mut vec![&s1, &s2]);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0.75),
            TypedVec::vector(0f64, 0f64, -1f64),
        );
        assert_eq!(
            w.colour_at(r).round(100000f64),
            w.objects[1].material().colour
        )
    }

    #[test]
    fn test_no_shadow() {
        default_world!(w, s1, s2);
        let p = TypedVec::point(0f64, 10f64, 0f64);
        assert!(!w.is_shadowed(p))
    }

    #[test]
    fn test_shadow_obj_between_light_and_point() {
        default_world!(w, s1, s2);
        let p = TypedVec::point(10f64, -10f64, 10f64);
        assert!(w.is_shadowed(p))
    }

    #[test]
    fn test_light_between_point_and_obj() {
        default_world!(w, s1, s2);
        let p = TypedVec::point(-20f64, 20f64, -20f64);
        assert!(!w.is_shadowed(p))
    }

    #[test]
    fn test_shadow_object_behind_point() {
        default_world!(w, s1, s2);
        let p = TypedVec::point(-2f64, 2f64, -2f64);
        assert!(!w.is_shadowed(p))
    }

    #[test]
    fn test_shade_hit_in_shadow() {
        let mut w = World::new(Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE));
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.transform = Some(Matrix::translation(0f64, 0f64, 10f64));
        w.objects = vec![&s1, &s2];
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let i = Intersection::new(4f64, &s2);
        let comps = i.precompute(r);
        assert_eq!(w.shade_hit(comps), Colour::new(0.1, 0.1, 0.1));
    }
}
