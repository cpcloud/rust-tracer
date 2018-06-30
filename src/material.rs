use hitrecord::HitRecord;
use ray::Ray;
use std::marker::Sync;
use utils;
use vec3::Vec3;

pub trait RawMaterial: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub type Material = Box<RawMaterial>;

#[derive(Debug, PartialEq)]
struct Lambertian {
    albedo: Vec3,
}

impl RawMaterial for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let point = rec.point();
        let target = point + rec.normal() + utils::random_in_unit_sphere();
        let scattered = Ray::new(point, target - point);
        Some((self.albedo, scattered))
    }
}

pub fn lambertian(albedo: Vec3) -> Material {
    box Lambertian { albedo }
}

#[derive(Debug, PartialEq)]
struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl RawMaterial for Metal {
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

pub fn metal(albedo: Vec3, fuzz: f64) -> Material {
    box Metal {
        albedo,
        fuzz: fuzz.min(1.0),
    }
}

#[derive(Debug, PartialEq)]
struct Dielectric {
    ref_idx: f64,
}

impl RawMaterial for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let dir = r_in.direction();
        let dir_length = dir.norm();
        let rec_normal = rec.normal();
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
        Some((vec3![1, 1, 1], Ray::new(rec.point(), direction)))
    }
}

pub fn dielectric(ref_idx: f64) -> Material {
    box Dielectric { ref_idx }
}
