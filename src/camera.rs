use crate::{
    ray::Ray,
    utils::rand,
    vec3::{GeomVec, Vec3},
};
use std::f64;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

fn random_in_unit_disk() -> Vec3 {
    let one_one_zero = vec3![1, 1, 0];
    let mut p = 2.0 * vec3![rand(), rand(), 0] - one_one_zero;
    while p.norm2() >= 1.0 {
        p = 2.0 * vec3![rand(), rand(), 0] - one_one_zero;
    }
    p
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = fov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unitize();
        let u = vup.cross(w).unitize();
        let v = w.cross(u);
        Camera {
            origin,
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            lens_radius,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let origin = self.origin;
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - origin - offset,
        )
    }
}
