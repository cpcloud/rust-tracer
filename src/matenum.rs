use hitrecord::HitRecord;
use ray::Ray;
use utils;
use vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Lambertian(albedo) => {
                let point = rec.point();
                let target = point + rec.normal() + utils::random_in_unit_sphere();
                let scattered = Ray::new(point, target - point);
                Some((albedo, scattered))
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = r_in.direction().unitize().reflect(rec.normal());
                let scattered = Ray::new(
                    rec.point(),
                    reflected + fuzz * utils::random_in_unit_sphere(),
                );
                if scattered.direction().dot(rec.normal()) > 0.0 {
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric(ref_idx) => {
                let dir = r_in.direction();
                let dir_length = dir.norm();
                let rec_normal = rec.normal();
                let reflected = dir.reflect(rec_normal);
                let dir_dot_normal = dir.dot(rec_normal);

                let (outward_normal, ni_over_nt, cosine) =
                    if r_in.direction().dot(rec.normal()) > 0.0 {
                        (-rec_normal, ref_idx, ref_idx * dir_dot_normal / dir_length)
                    } else {
                        (rec_normal, 1.0 / ref_idx, -dir_dot_normal / dir_length)
                    };

                let direction =
                    if let Some(refracted) = r_in.direction().refract(outward_normal, ni_over_nt) {
                        if utils::rand() < utils::schlick(cosine, ref_idx) {
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
    }
}

pub fn lambertian(albedo: Vec3) -> Material {
    Material::Lambertian(albedo)
}

pub fn metal(albedo: Vec3, fuzz: f64) -> Material {
    Material::Metal(albedo, fuzz)
}

pub fn dielectric(ref_idx: f64) -> Material {
    Material::Dielectric(ref_idx)
}
