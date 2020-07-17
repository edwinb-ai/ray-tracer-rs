use ray_tracer::tuple::{Point, Tuple, Vector};
use ray_tracer::{point, vector};

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;

    Projectile {
        position: pos,
        velocity: vel,
    }
}

fn main() {
    let pos = point!(0, 1, 0);
    let vel = vector!(1, 1, 0);
    let p = Projectile {
        position: pos,
        velocity: vel.normalize() * 10.5,
    };

    let e = Environment {
        gravity: vector!(0, -0.1, 0),
        wind: vector!(-0.02, 0, 0),
    };

    let mut p1: Projectile = tick(&e, &p);
    let mut counter: usize = 0;

    while p1.position.get_y() >= 0.0 && p1.position.get_x() >= 0.0 {
        p1 = tick(&e, &p1);
        counter += 1;

        println!("{:?} {}", p1, counter);
    }
}
