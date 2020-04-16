use ray_trace_challenge::canvas::Canvas;
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

    let mut canvas = Canvas::new(900, 550);

    let mut p = Projectile {
        position: TypedVec::point(0.0, 1.0, 0.0),
        velocity: TypedVec::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: TypedVec::vector(0.0, -0.1, 0.0),
        wind: TypedVec::vector(-0.01, 0.0, 0.0),
    };

    let mut ticks = 0;
    while p.position.y > 0.0 {
        p = tick(&e, p, &mut canvas);
        ticks += 1;
    }
    println!("Total ticks: {}", ticks);
    let mut out = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("canvas.ppm")
        .unwrap();
    out.write_all(canvas.save().as_bytes()).unwrap();
}
