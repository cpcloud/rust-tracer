use crate::{ray::Ray, utils, vec3::Vec3, HitRecord};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Debug, PartialEq)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let point = rec.point;
        let target = point + rec.normal + utils::random_in_unit_sphere();
        let scattered = Ray::new(point, target - point);
        Some((self.albedo, scattered))
    }
}

#[derive(Debug, PartialEq)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction().unitize().reflect(rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * utils::random_in_unit_sphere(),
        );
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let dir = r_in.direction();
        let dir_length = dir.norm();
        let rec_normal = rec.normal;
        let reflected = dir.reflect(rec_normal);
        let dir_dot_normal = dir.dot(rec_normal);
        let ref_idx = self.ref_idx;

        let (outward_normal, ni_over_nt, factor) = if dir_dot_normal > 0.0 {
            (-rec_normal, ref_idx, ref_idx)
        } else {
            (rec_normal, 1.0 / ref_idx, -1.0)
        };

        let direction = if let Some(refracted) = dir.refract(outward_normal, ni_over_nt) {
            if utils::rand() < utils::schlick(factor * dir_dot_normal / dir_length, ref_idx) {
                reflected
            } else {
                refracted
            }
        } else {
            reflected
        };
        Some((vec3![1, 1, 1], Ray::new(rec.point, direction)))
    }
}
