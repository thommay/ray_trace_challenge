use crate::colour::{Colour, BLACK};
use crate::hittable::Hittable;
use crate::intersection::{Intersection, Intersections, PreComp};
use crate::lighting::Point;
use crate::ray::Ray;
use crate::vec3::TypedVec;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct World<'a> {
    light: Point,
    pub objects: Vec<&'a dyn Hittable>,
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

    fn shade_hit(&self, comps: PreComp, remaining: usize) -> Colour {
        let shadowed = self.is_shadowed(comps.over_point);
        let surface = comps.obj.material().lighting(
            comps.obj,
            self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_colour(comps.clone(), remaining);
        let refracted = self.refracted_colour(comps.clone(), remaining);
        let m = comps.obj.material();
        if m.reflective > 0f64 && m.transparency > 0f64 {
            let r = comps.schlick();
            surface + reflected * r + refracted * (1f64 - r)
        } else {
            surface + reflected + refracted
        }
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

    pub fn colour_at(&self, ray: Ray, remaining: usize) -> Colour {
        let xs = Intersections::from_iter(self.intersect(ray));
        xs.clone().hit().map_or_else(
            || *crate::colour::BLACK,
            |x| self.shade_hit(x.precompute(ray, &xs), remaining),
        )
    }

    fn reflected_colour(&self, comps: PreComp, remaining: usize) -> Colour {
        if remaining < 1 || comps.obj.material().reflective == 0f64 {
            return *BLACK;
        }
        let r = Ray::new(comps.over_point, comps.reflectv);
        let colour = self.colour_at(r, remaining - 1);
        colour * comps.obj.material().reflective
    }

    fn refracted_colour(&self, comps: PreComp, remaining: usize) -> Colour {
        if remaining < 1 || comps.obj.material().transparency == 0f64 {
            return *BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot_product(comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1f64 - cos_i.powi(2));
        if sin2_t > 1f64 {
            return *BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract = Ray::new(comps.under_point, direction);
        self.colour_at(refract, remaining - 1) * comps.obj.material().transparency
    }
}

#[cfg(test)]
pub mod test {
    use crate::colour::{Colour, BLACK, WHITE};
    use crate::intersection::{Intersection, Intersections};
    use crate::lighting;
    use crate::lighting::Point;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::pattern::Pattern;
    use crate::plane::Plane;
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
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        let c = w.shade_hit(comps, 4);
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
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        let c = w.shade_hit(comps, 4);
        assert_eq!(c.round(100000f64), Colour::new(0.38066, 0.47582, 0.28549));
    }

    #[test]
    fn test_miss() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        assert_eq!(w.colour_at(r, 4), Colour::new(0f64, 0f64, 0f64))
    }

    #[test]
    fn test_hit() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        assert_eq!(
            w.colour_at(r, 4).round(100000f64),
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
            w.colour_at(r, 4).round(100000f64),
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
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(w.shade_hit(comps, 4), Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_nonreflective_colour() {
        let mut w = World::new(Point::new(TypedVec::point(0f64, 0f64, -10f64), *WHITE));
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.transform = Some(Matrix::scaling(0.5, 0.5, 0.5));
        s2.material.ambient = 1f64;
        w.objects = vec![&s1, &s2];
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let i = Intersection::new(1f64, &s2);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(w.reflected_colour(comps, 1), *BLACK);
    }

    #[test]
    fn test_reflective_colour() {
        default_world!(w, s1, s2);
        let mut p = Plane::default();
        p.material.reflective = 0.5;
        p.transform = Some(Matrix::translation(0f64, -1f64, 0f64));
        w.objects.push(&p);

        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -3f64),
            TypedVec::vector(0f64, -2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64),
        );
        let i = Intersection::new(2f64.sqrt(), &p);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(
            w.reflected_colour(comps, 1).round(100000f64),
            Colour::new(0.19035, 0.23793, 0.14276)
        );
    }

    #[test]
    fn test_reflective_shade_hit() {
        default_world!(w, s1, s2);
        let mut p = Plane::default();
        p.material.reflective = 0.5;
        p.transform = Some(Matrix::translation(0f64, -1f64, 0f64));
        w.objects.push(&p);

        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -3f64),
            TypedVec::vector(0f64, -2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64),
        );
        let i = Intersection::new(2f64.sqrt(), &p);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(
            w.shade_hit(comps, 4).round(100000f64),
            Colour::new(0.87677, 0.92436, 0.82918)
        );
    }

    #[test]
    fn test_mutually_reflective() {
        let mut w = World::new(Point::new(TypedVec::point(0f64, 0f64, 0f64), *WHITE));

        let lower = {
            let mut p = Plane::default();
            p.material.reflective = 1f64;
            p.transform = Some(Matrix::translation(0f64, -1f64, 0f64));
            p
        };

        let upper = {
            let mut p = Plane::default();
            p.material.reflective = 1f64;
            p.transform = Some(Matrix::translation(0f64, 1f64, 0f64));
            p
        };

        w.objects = vec![&lower, &upper];

        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );

        w.colour_at(r, 4);
        assert!(true)
    }

    #[test]
    fn test_reflective_shade_hit_at_maximum_recursion() {
        default_world!(w, s1, s2);
        let mut p = Plane::default();
        p.material.reflective = 0.5;
        p.transform = Some(Matrix::translation(0f64, -1f64, 0f64));
        w.objects.push(&p);

        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -3f64),
            TypedVec::vector(0f64, -2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64),
        );
        let i = Intersection::new(2f64.sqrt(), &p);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(
            w.reflected_colour(comps, 4).round(100000f64),
            Colour {
                red: 0.19035,
                green: 0.23793,
                blue: 0.14276
            }
        );
    }

    #[test]
    fn test_refracted_opaque() {
        default_world!(w, s1, s2);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );

        let x = vec![Intersection::new(4f64, &s1), Intersection::new(6f64, &s1)];
        let xs = Intersections::from_iter(x);
        let comps = xs[0].precompute(r, &xs);
        let c = w.refracted_colour(comps, 5);
        assert_eq!(c, *BLACK);
    }

    #[test]
    fn test_refracted_max() {
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1f64,
                transparency: 1.0,
                refractive_index: 1.5,
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
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );

        let x = vec![Intersection::new(4f64, &s1), Intersection::new(6f64, &s1)];
        let xs = Intersections::from_iter(x);
        let comps = xs[0].precompute(r, &xs);
        let c = w.refracted_colour(comps, 0);
        assert_eq!(c, *BLACK);
    }

    #[test]
    fn test_refracted_total_internal() {
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1f64,
                transparency: 1.0,
                refractive_index: 1.5,
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
            TypedVec::point(0f64, 0f64, 2f64.sqrt() / 2f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );

        let x = vec![
            Intersection::new(-2f64.sqrt() / 2f64, &s1),
            Intersection::new(2f64.sqrt() / 2f64, &s1),
        ];
        let xs = Intersections::from_iter(x);
        let comps = xs[1].precompute(r, &xs);
        let c = w.refracted_colour(comps, 5);
        assert_eq!(c, *BLACK);
    }

    #[test]
    fn test_refracted_ray() {
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ambient: 1f64,
                pattern: Some(Pattern::default()),
                ..Default::default()
            },
            ..Default::default()
        };
        let s2 = Sphere {
            transform: Some(Matrix::scaling(0.5, 0.5, 0.5)),
            material: Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut w = World::default();
        w.objects.append(&mut vec![&s1, &s2]);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0.1),
            TypedVec::vector(0f64, 1f64, 0f64),
        );

        let x = vec![
            Intersection::new(-0.9899, &s1),
            Intersection::new(-0.4899, &s2),
            Intersection::new(0.4899, &s2),
            Intersection::new(0.9899, &s1),
        ];
        let xs = Intersections::from_iter(x);
        let comps = xs[2].precompute(r, &xs);
        let c = w.refracted_colour(comps, 5);
        assert_eq!(c, Colour::new(0f64, 0.99888, 0.04725));
    }

    #[test]
    fn test_shade_trans() {
        default_world!(w, s1, s2);
        let floor = Plane {
            transform: Some(Matrix::translation(0f64, -1f64, 0f64)),
            material: Material {
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            },
        };
        let ball = Sphere {
            transform: Some(Matrix::translation(0f64, -3.5f64, -0.5f64)),
            material: Material {
                colour: Colour::new(1f64, 0f64, 0f64),
                ambient: 0.5,
                ..Default::default()
            },
        };
        w.objects.push(&floor);
        w.objects.push(&ball);

        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -3f64),
            TypedVec::vector(0f64, -2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64),
        );

        let i = Intersection::new(2f64.sqrt(), &floor);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(
            w.shade_hit(comps, 5).round(100_000f64),
            Colour::new(0.93642, 0.68642, 0.68642)
        )
    }

    #[test]
    fn test_shade_reflective_trans() {
        default_world!(w, s1, s2);
        let floor = Plane {
            transform: Some(Matrix::translation(0f64, -1f64, 0f64)),
            material: Material {
                reflective: 0.5,
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            },
        };
        let ball = Sphere {
            transform: Some(Matrix::translation(0f64, -3.5f64, -0.5f64)),
            material: Material {
                colour: Colour::new(1f64, 0f64, 0f64),
                ambient: 0.5,
                ..Default::default()
            },
        };
        w.objects.push(&floor);
        w.objects.push(&ball);

        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -3f64),
            TypedVec::vector(0f64, -2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64),
        );

        let i = Intersection::new(2f64.sqrt(), &floor);
        let xs = Intersections::from_iter(vec![i.clone()]);
        let comps = i.precompute(r, &xs);
        assert_eq!(
            w.shade_hit(comps, 5).round(100_000f64),
            Colour::new(0.93391, 0.69643, 0.69243)
        )
    }
}
