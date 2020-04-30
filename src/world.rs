use crate::colour::Colour;
use crate::intersection::{Intersection, Intersections, PreComp};
use crate::lighting::Point;
use crate::ray::Ray;
use crate::sphere::Hittable;
use crate::vec3::TypedVec;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct World<T>
where
    T: Hittable + PartialOrd + Debug + Clone,
{
    light: Point,
    objects: Vec<T>,
}

impl<T> Default for World<T>
where
    T: Hittable + PartialOrd + Debug + Clone,
{
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

impl<T> World<T>
where
    T: Hittable<Output = T> + PartialOrd + Debug + Clone,
{
    pub fn new(light: Point) -> Self {
        World {
            light,
            objects: Vec::new(),
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection<T>> {
        let mut r: Vec<Intersection<T>> =
            self.objects.iter().flat_map(|o| o.intersect(ray)).collect();
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        r
    }

    fn shade_hit(&self, comps: PreComp<T>) -> Colour {
        comps
            .obj
            .material()
            .lighting(self.light, comps.point, comps.eyev, comps.normalv)
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
mod test {
    use crate::colour::{Colour, WHITE};
    use crate::intersection::{Intersection, Intersections};
    use crate::lighting;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::vec3::TypedVec;
    use crate::world::World;

    fn default_world() -> World<Sphere> {
        let s1 = Sphere {
            material: Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        };
        let s2 = Sphere {
            transform: Some(Matrix::scaling(0.5, 0.5, 0.5)),
            ..Default::default()
        };

        let mut w = World::default();
        w.objects.append(&mut vec![s1, s2]);
        w
    }

    #[test]
    fn test_world_intersect() {
        let w = default_world();
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
        let mut w = default_world();
        w.light = lighting::Point::new(TypedVec::point(0f64, 0.25, 0f64), *WHITE);
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -0f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.precompute(r);
        let c = w.shade_hit(comps);
        assert_eq!(c.round(100000f64), Colour::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_shading() {
        let w = default_world();
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        let shape = &w.objects[0];
        let i = Intersection::new(4f64, shape);
        let comps = i.precompute(r);
        let c = w.shade_hit(comps);
        assert_eq!(c.round(100000f64), Colour::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_miss() {
        let w = default_world();
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 1f64, 0f64),
        );
        assert_eq!(w.colour_at(r), Colour::new(0f64, 0f64, 0f64))
    }

    #[test]
    fn test_hit() {
        let w = default_world();
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, -5f64),
            TypedVec::vector(0f64, 0f64, 1f64),
        );
        assert_eq!(
            w.colour_at(r).round(100000f64),
            Colour::new(0.38066, 0.47583, 0.2855)
        )
    }

    #[test]
    fn test_hit_behind() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1f64;
        w.objects[1].material.ambient = 1f64;
        let r = Ray::new(
            TypedVec::point(0f64, 0f64, 0.75),
            TypedVec::vector(0f64, 0f64, -1f64),
        );
        assert_eq!(
            w.colour_at(r).round(100000f64),
            w.objects[1].material.colour
        )
    }
}
