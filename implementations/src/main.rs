use ray_tracer::tuple::{Tuple, Vector};
use ray_tracer::vector;

fn main() {
    let v1 = vector!(1, 2, 3);
    let v2 = vector!(2, 3, 4);
    let res= vector!(-1, 2, -1);
    let validation: bool = v1.cross(&v2) == res;

    println!("{}", validation);
}
