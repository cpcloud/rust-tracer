use hitrecord::HitRecord;
use ray::Ray;
use std::marker::Sync;
use utils;
use vec3::Vec3;

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Debug)]
struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let point = rec.point();
        let target = point + rec.normal() + utils::random_in_unit_sphere();
        let scattered = Ray::new(point, target - point);
        Some((self.albedo, scattered))
    }
}

pub fn lambertian(albedo: Vec3) -> Box<Material> {
    box Lambertian::new(albedo)
}

#[derive(Debug)]
struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction().unitize().reflect(rec.normal());
        let scattered = Ray::new(
            rec.point(),
            reflected + self.fuzz * utils::random_in_unit_sphere(),
        );
        if scattered.direction().dot(rec.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub fn metal(albedo: Vec3, fuzz: f64) -> Box<Material> {
    box Metal::new(albedo, fuzz)
}

#[derive(Debug)]
struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let dir = r_in.direction();
        let dir_length = dir.norm();
        let rec_normal = rec.normal();
        let reflected = dir.reflect(rec_normal);
        let ref_idx = self.ref_idx;

        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(rec.normal()) > 0.0 {
            (
                -rec_normal,
                ref_idx,
                ref_idx * dir.dot(rec_normal) / dir_length,
            )
        } else {
            (rec_normal, 1.0 / ref_idx, -dir.dot(rec_normal) / dir_length)
        };

        let direction =
            if let Some(refracted) = r_in.direction().refract(outward_normal, ni_over_nt) {
                if utils::rand() < schlick(cosine, self.ref_idx) {
                    reflected
                } else {
                    refracted
                }
            } else {
                reflected
            };
        Some((vec3![1, 1, 1], Ray::new(rec.point(), direction)))
    }
}

pub fn dielectric(ref_idx: f64) -> Box<Material> {
    box Dielectric::new(ref_idx)
}
