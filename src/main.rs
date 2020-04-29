use ray_trace_challenge::canvas::Canvas;
use ray_trace_challenge::colour::{Colour, WHITE};
use ray_trace_challenge::intersection::Intersections;
use ray_trace_challenge::lighting;
use ray_trace_challenge::material::Material;
use ray_trace_challenge::matrix::Matrix;
use ray_trace_challenge::ray::Ray;
use ray_trace_challenge::sphere::{Hittable, Sphere};
use ray_trace_challenge::vec3::TypedVec;
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
    fn tick(env: &Environment, proj: Projectile, canvas: &mut Canvas) -> Projectile {
        let position = proj.position + proj.velocity;
        let velocity = proj.velocity + env.gravity + env.wind;
        let colour = Colour::new(1f64, 0.0, 0.0);
        let y = if position.y.round() as usize > canvas.height {
            canvas.height
        } else {
            canvas.height - position.y.round() as usize
        };
        let x = position.x.round() as usize;
        dbg!(&x);
        dbg!(&y);
        canvas.write_pixel(x, y, colour);
        Projectile { velocity, position }
    }

    let mut canvas = Canvas::new(100, 100);

    let ray_origin = TypedVec::point(0f64, 0f64, -5f64);
    let wall_size = 7f64;
    let pixel_size = wall_size / 100f64;
    let half = wall_size / 2.0;
    let mut s = Sphere::new();
    // let t =
    //     Matrix::shearing(1f64, 0f64, 0f64, 0f64, 0f64, 0f64) * Matrix::scaling(0.5f64, 1f64, 1f64);
    // s.set_transform(t);
    s.set_material(Material {
        colour: Colour::new(1f64, 0.2, 1f64),
        ..Default::default()
    });

    let l = lighting::Point::new(TypedVec::point(-10f64, 10f64, -10f64), *WHITE);
    for y in 0..100 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..100 {
            let world_x = -half + pixel_size * x as f64;
            let pos = TypedVec::point(world_x, world_y, 10f64);
            let r = Ray::new(ray_origin, (pos - ray_origin).normalize());
            let xs = s.intersect(r);
            let mut xs = Intersections::from_iter(xs);
            if let Some(hit) = xs.hit() {
                let point = r.position(hit.t);
                let n = hit.obj.normal_at(point).unwrap();
                let eye = -r.direction;
                let colour = hit.obj.material.lighting(l, point, eye, n);
                canvas.write_pixel(x as usize, y as usize, colour);
            }
        }
    }
    // // let mut p = Projectile {
    // //     position: TypedVec::point(0.0, 1.0, 0.0),
    // //     velocity: TypedVec::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    // // };
    // // let e = Environment {
    // //     gravity: TypedVec::vector(0.0, -0.1, 0.0),
    // //     wind: TypedVec::vector(-0.01, 0.0, 0.0),
    // // };
    // //
    // let p = TypedVec::point(0f64, 0f64, 16f64);
    // let centre = TypedVec::vector(25f64, 25f64, 25f64);
    // for i in 0..12 {
    //     let r = Matrix::rotation(Axis::Y, i as f64 * (std::f64::consts::PI / 6f64));
    //     let h = r * p;
    //     let centred = h + centre;
    //     dbg!(&centred);
    //     canvas.write_pixel(
    //         centred.x.round() as usize,
    //         centred.z.round() as usize,
    //         colour,
    //     );
    // }
    //
    // let mut ticks = 0;
    // while p.position.y > 0.0 {
    //     p = tick(&e, p, &mut canvas);
    //     ticks += 1;
    // }
    // println!("Total ticks: {}", ticks);
    //
    let mut out = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("canvas.ppm")
        .unwrap();
    out.write_all(canvas.save().as_bytes()).unwrap();
}
