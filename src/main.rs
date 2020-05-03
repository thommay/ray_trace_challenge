use ray_trace_challenge::camera::{view_transform, Camera};
use ray_trace_challenge::colour::{Colour, WHITE};
use ray_trace_challenge::lighting::Point;
use ray_trace_challenge::matrix::{Axis, Matrix};
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
    let mut world = World::new(Point::new(TypedVec::point(-10f64, 10f64, -10f64), *WHITE));

    let mut floor = Sphere::new();
    floor.transform = Some(Matrix::scaling(10f64, 0.01, 10f64));
    floor.material.colour = Colour::new(1f64, 0.9f64, 0.9f64);
    floor.material.specular = 0f64;

    let mut left_wall = Sphere::new();
    left_wall.transform = Some(
        Matrix::translation(0f64, 0f64, 5f64)
            * Matrix::rotation(Axis::Y, -PI / 4f64)
            * Matrix::rotation(Axis::X, PI / 2f64)
            * Matrix::scaling(10f64, 0.01f64, 10f64),
    );
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new();
    right_wall.transform = Some(
        Matrix::translation(0f64, 0f64, 5f64)
            * Matrix::rotation(Axis::Y, PI / 4f64)
            * Matrix::rotation(Axis::X, PI / 2f64)
            * Matrix::scaling(10f64, 0.01f64, 10f64),
    );
    right_wall.material = floor.material;

    let mut middle = Sphere::new();
    middle.transform = Some(Matrix::translation(-0.5, 1f64, 0.5));
    middle.material.colour = Colour::new(0.1, 1f64, 0.1);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = Some(Matrix::translation(1.5, 0.5f64, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    right.material.colour = Colour::new(0.1, 1f64, 0.5);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform =
        Some(Matrix::translation(-1.5, 0.33f64, -0.75) * Matrix::scaling(0.33, 0.33, 0.33));
    left.material.colour = Colour::new(1f64, 0.8f64, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut items = vec![floor, left_wall, right_wall, middle, right, left];
    world.objects.append(&mut items);

    let mut camera = Camera::new(500f64, 250f64, PI / 3f64);
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
