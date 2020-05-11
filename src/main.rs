use ray_trace_challenge::camera::{view_transform, Camera};
use ray_trace_challenge::colour::*;
use ray_trace_challenge::lighting::Point;
use ray_trace_challenge::matrix::{Axis, Matrix};
use ray_trace_challenge::pattern::Pattern;
use ray_trace_challenge::pattern::PatternType::{Checker, Stripe};
use ray_trace_challenge::plane::Plane;
use ray_trace_challenge::sphere::{HittableImpl, Sphere};
use ray_trace_challenge::vec3::TypedVec;
use ray_trace_challenge::world::World;
use std::f64::consts::PI;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone)]
struct Projectile {
    velocity: TypedVec,
    position: TypedVec,
}

#[derive(Debug, Clone)]
struct Environment {
    gravity: TypedVec,
    wind: TypedVec,
}

fn main() {
    let mut world = World::new(Point::new(TypedVec::point(-10f64, 2f64, -10f64), *WHITE));

    let mut plane = Plane::default();
    plane.material.pattern = Some(Pattern::new(Checker, *WHITE, *BLACK, false));

    let mut ceiling = Plane::default();
    ceiling.transform = Some(
        Matrix::translation(0f64, 2.5f64, 0f64)
            * Matrix::rotation(Axis::Y, PI / 4f64)
            * Matrix::scaling(1f64, 10f64, 1f64),
    );
    ceiling.material.colour = Colour::new(1f64, 0.3f64, 1f64);
    // ceiling.material.pattern = Some(Pattern::new(Stripe, *WHITE, *BLACK));

    let mut back_wall = Plane::default();
    back_wall.transform = Some(
        Matrix::translation(0f64, 0f64, 5f64)
            * Matrix::rotation(Axis::X, PI / 2f64)
            * Matrix::scaling(1f64, 10f64, 1f64),
    );
    back_wall.material.ambient = 0.5f64;
    back_wall.material.pattern = Some(Pattern::new(Stripe, *WHITE, *BLACK, false));

    let mut middle = Sphere::new();
    middle.transform = Some(Matrix::translation(-0.5, 1f64, 0.5));
    middle.material.colour = Colour::new(0.1, 1f64, 0.1);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let mut p = Pattern::ring(*WHITE, Colour::new(0.1f64, 0.8f64, 0.1), true);
    p.set_transform(Matrix::scaling(0.5, 0.5, 0.5) * Matrix::rotation(Axis::Z, PI / 4f64));

    middle.material.pattern = Some(p);

    let mut right = Sphere::new();
    right.transform = Some(Matrix::translation(1.5, 0.5f64, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    right.material.colour = Colour::new(0.1, 1f64, 0.5);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.pattern = Some({
        let mut p = Pattern::ring(*WHITE, right.material.colour, true);
        p.set_transform(Matrix::scaling(0.15, 0.15, 0.25));
        p
    });

    let mut left = Sphere::new();
    left.transform =
        Some(Matrix::translation(-1.7, 0.33f64, -0.75) * Matrix::scaling(0.33, 0.33, 0.33));
    left.material.colour = Colour::new(1f64, 0.8f64, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut items: Vec<&dyn HittableImpl> = vec![&plane, &back_wall, &middle, &right, &left];
    world.objects.append(&mut items);

    // Good
    // let mut camera = Camera::new(1000f64, 500f64, PI / 3f64);
    // Medium
    let mut camera = Camera::new(500f64, 250f64, PI / 3f64);
    // Quick
    // let mut camera = Camera::new(100f64, 50f64, PI / 3f64);
    camera.transform = view_transform(
        TypedVec::point(0f64, 1.5, -5f64),
        TypedVec::point(0f64, 1f64, 0f64),
        TypedVec::vector(0f64, 1f64, 0f64),
    );

    let canvas = camera.render(world);

    let mut out = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("canvas.ppm")
        .unwrap();
    out.write_all(canvas.save().as_bytes()).unwrap();
}
