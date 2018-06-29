extern crate rand;

use self::rand::{Rng, SeedableRng, StdRng};

use std::sync::Mutex;

use vec3::Vec3;

lazy_static! {
    static ref RNG: Mutex<StdRng> = Mutex::new(SeedableRng::from_seed([0u8; 32]));
}

pub fn rand() -> f64 {
    RNG.lock().unwrap().gen()
    //    RNG.sample_iter(&Standard).take(5).collect::<Vec<(f64, f64)>>()
}

pub fn randvec() -> Vec3 {
    vec3![rand(), rand(), rand()]
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * randvec() - Vec3::ones();
        if p.norm2() < 1.0 {
            return p;
        }
    }
}
