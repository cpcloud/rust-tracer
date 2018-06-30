use rand;

use vec3::Vec3;

pub fn rand() -> f64 {
    rand::random()
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

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
