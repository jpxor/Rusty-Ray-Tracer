
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::cell::UnsafeCell;
use std::rc::Rc;
use cgmath::InnerSpace;
type Vector3 = cgmath::Vector3<f32>;
use lazy_static::lazy_static;

// creating a thread local version of SmallRng using the same
// technique as thread_rng as see here:
// https://docs.rs/rand/0.8.5/src/rand/rngs/thread.rs.html#67-91
thread_local! {
    static RNG: Rc<UnsafeCell<SmallRng>> = Rc::new(UnsafeCell::new(SmallRng::seed_from_u64(0)));
}

lazy_static! {
    static ref RAND_UNIT_VECTOR_LUT:Vec<Vector3> = (0..128).map( |_| {
        random_unit_vector3()
    }).collect();
}

pub fn random_unit_vector3() -> Vector3 {
    random_in_unit_sphere().normalize()
}

fn random_in_unit_sphere() -> Vector3 {
    let rng = RNG.with(|t| t.clone());

    // SAFETY: for thread local use only
    let rng = unsafe { &mut *rng.get() };

    loop {
        let v = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if cgmath::dot(v, v) > 1.0 {
            continue
        } else {
            return v;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3 {
    let rng = RNG.with(|t| t.clone());

    // SAFETY: for thread local use only
    let rng = unsafe { &mut *rng.get() };

    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            0.0,
        );
        if cgmath::dot(p, p) > 1.0 {
            continue
        } else {
            return p;
        }
    }
}
