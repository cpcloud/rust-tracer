use crate::hitrecord::HitRecord;
use crate::mat::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64;
use std::marker::Sync;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let radius = self.radius;
        let center = self.center;

        let oc = ray.origin() - center;
        let dir = ray.direction();
        let a = dir.norm2();
        let b = oc.dot(dir);
        let c = oc.norm2() - radius.powi(2);
        let disc = b.powi(2) - a * c;
        if disc > 0.0 {
            let disc_sqrt = disc.sqrt();
            let t = (-b - disc_sqrt) / a;
            if t < t_max && t > t_min {
                let point = ray.point(t);
                Some(HitRecord::new(
                    t,
                    point,
                    (point - center) / radius,
                    &self.material,
                ))
            } else {
                let t = (-b + disc_sqrt) / a;
                if t < t_max && t > t_min {
                    let point = ray.point(t);
                    Some(HitRecord::new(
                        t,
                        point,
                        (point - center) / radius,
                        &self.material,
                    ))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

pub fn sphere(
    center: Vec3,
    radius: f64,
    material: Material,
) -> Box<dyn Hittable> {
    box Sphere {
        center,
        radius,
        material,
    }
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;
        for item in self.iter() {
            if let Some(temp_rec) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t();
                rec = Some(temp_rec);
            }
        }
        rec
    }
}
