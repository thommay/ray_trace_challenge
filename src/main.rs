use ray_trace_challenge::camera::{view_transform, Camera};
use ray_trace_challenge::colour::*;
use ray_trace_challenge::cone::Cone;
use ray_trace_challenge::cube::Cube;
use ray_trace_challenge::cylinder::Cylinder;
use ray_trace_challenge::hittable::Hittable;
use ray_trace_challenge::lighting::Point;
use ray_trace_challenge::matrix::{Axis, Matrix};
use ray_trace_challenge::pattern::Pattern;
use ray_trace_challenge::pattern::PatternType::Stripe;
use ray_trace_challenge::plane::Plane;
use ray_trace_challenge::sphere::Sphere;
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
    plane.material.reflective = 0.7;
    plane.material.specular = 0.3;
    //
    // let mut ceiling = Plane::default();
    // ceiling.transform = Some(
    //     Matrix::translation(0f64, 2.5f64, 0f64)
    //         * Matrix::rotation(Axis::Y, PI / 4f64)
    //         * Matrix::scaling(1f64, 10f64, 1f64),
    // );
    // ceiling.material.colour = Colour::new(1f64, 0.3f64, 1f64);
    // // ceiling.material.pattern = Some(Pattern::new(Stripe, *WHITE, *BLACK));
    //
    let mut back_wall = Plane::default();
    back_wall.transform = Some(
        Matrix::translation(0f64, 0f64, 5f64)
            * Matrix::rotation(Axis::X, PI / 2f64)
            * Matrix::scaling(1f64, 10f64, 1f64),
    );
    back_wall.material.ambient = 0.5f64;
    // back_wall.material.pattern = Some(Pattern::new(Stripe, *WHITE, *BLACK, false));
    //
    // let mut middle = Sphere::glass();
    // middle.transform = Some(Matrix::translation(-0.5, 1f64, 0.5));
    // middle.material.colour = Colour::new(0.1, 1f64, 0.1);
    // middle.material.reflective = 0.9;
    // middle.material.diffuse = 0.5;
    //
    // // let mut p = Pattern::ring(*WHITE, Colour::new(0.1f64, 0.8f64, 0.1), true);
    // // p.set_transform(Matrix::scaling(0.5, 0.5, 0.5) * Matrix::rotation(Axis::Z, PI / 4f64));
    // // middle.material.pattern = Some(p);
    // //
    // let mut right = Cylinder::default();
    // right.minimum = 1.0;
    // right.maximum = 2.0;
    // right.closed = true;
    // right.transform = Some(
    //     Matrix::translation(1.5, 0.5f64, -0.5)
    //         * Matrix::scaling(0.5, 0.5, 0.5)
    //         * Matrix::rotation(Axis::Y, 1.5)
    //         * Matrix::rotation(Axis::Z, 1.0),
    // );
    // right.material.colour = Colour::new(0.1, 1f64, 0.1);
    // right.material.diffuse = 0.7;
    // right.material.specular = 1f64;
    // right.material.shininess = 300f64;
    //
    // right.material.reflective = 0.8;
    // // right.material.transparency = 0.9;
    // // right.material.pattern = Some({
    // //     let mut p = Pattern::ring(*WHITE, right.material.colour, true);
    // //     p.set_transform(Matrix::scaling(0.15, 0.15, 0.25));
    // //     p
    // // });
    //
    // let mut left = Sphere::new();
    // left.transform =
    //     Some(Matrix::translation(-1.7, 0.33f64, -0.75) * Matrix::scaling(0.33, 0.33, 0.33));
    // left.material.colour = Colour::new(1f64, 0.8f64, 0.1);
    // left.material.diffuse = 0.7;
    // left.material.specular = 0.3;
    //
    // let mut cube = Cube::default();
    // cube.transform =
    //     Some(Matrix::translation(-1.2, 0.5f64, -0.75) * Matrix::scaling(0.25, 0.25, 0.25));
    //
    // let mut items: Vec<&dyn HittableImpl> = vec![&plane, &back_wall, &middle, &right, &left, &cube];

    let mut cone = Cone {
        minimum: -1.0,
        maximum: 0.0,
        ..Default::default()
    };
    cone.material.reflective = 1.0;
    cone.material.specular = 1f64;
    cone.transform = Some(Matrix::translation(0.0, 2.0, 0.0) * Matrix::scaling(0.25, 0.25, 0.25));

    let mut top = Cylinder {
        maximum: 1.0,
        minimum: 0.0,
        ..Default::default()
    };
    top.material.reflective = 1.0;
    top.material.specular = 1f64;
    top.transform = Some(Matrix::translation(0.0, 1.5, 0.0) * Matrix::scaling(0.25, 0.25, 0.25));
    let mut items: Vec<&dyn Hittable> = vec![&plane, &back_wall, &cone, &top];
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
