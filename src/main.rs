use ray_trace_challenge::canvas::Canvas;
use ray_trace_challenge::matrix::{Axis, Matrix};
use ray_trace_challenge::vec3::TypedVec;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone)]
struct Projectile {
    velocity: TypedVec<f64>,
    position: TypedVec<f64>,
}

#[derive(Debug, Clone)]
struct Environment {
    gravity: TypedVec<f64>,
    wind: TypedVec<f64>,
}

fn main() {
    fn tick(env: &Environment, proj: Projectile, canvas: &mut Canvas) -> Projectile {
        let position = proj.position + proj.velocity;
        let velocity = proj.velocity + env.gravity + env.wind;
        let colour = TypedVec::colour(1.0, 0.0, 0.0);
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

    let mut canvas = Canvas::new(50, 50);
    let colour = TypedVec::colour(1.0, 0.0, 0.0);

    // let mut p = Projectile {
    //     position: TypedVec::point(0.0, 1.0, 0.0),
    //     velocity: TypedVec::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    // };
    // let e = Environment {
    //     gravity: TypedVec::vector(0.0, -0.1, 0.0),
    //     wind: TypedVec::vector(-0.01, 0.0, 0.0),
    // };
    //
    let p = TypedVec::point(0f64, 0f64, 16f64);
    let centre = TypedVec::vector(25f64, 25f64, 25f64);
    for i in 0..12 {
        let r = Matrix::rotation(Axis::Y, i as f64 * (std::f64::consts::PI / 6f64));
        let h = r * p;
        let centred = h + centre;
        dbg!(&centred);
        canvas.write_pixel(
            centred.x.round() as usize,
            centred.z.round() as usize,
            colour,
        );
    }

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
