use ray_tracer::tuple::{Tuple, Point, Vector};
use ray_tracer::{vector, point};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;

    Projectile { position:pos, velocity:vel }
}

fn main() {
    let v = vector!(1, 0, 0);
    let p = Projectile {
        position: point!(0, 1, 0),
        velocity: v.normalize()
    };

    let e = Environment {
        gravity: vector!(0, -0.1, 0),
        wind: vector!(-0.01, 0, 0),
    };

    let p1 = tick(&e, &p);
    let res = Projectile {
        position: point!(1, 1, 0),
        velocity: vector!(0.99, -0.1, 0),
    };

    assert!(res.position == p1.position);
    assert!(res.velocity == p1.velocity);
}
