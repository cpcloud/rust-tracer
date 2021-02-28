use crate::{HitRecord, Material, Ray, Vec3};

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: impl Material + Sync + 'static) -> Self {
        Self {
            center,
            radius,
            material: Box::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                Some(HitRecord {
                    t,
                    point,
                    normal: (point - center) / radius,
                    material: self.material.as_ref(),
                })
            } else {
                let t = (-b + disc_sqrt) / a;
                if t < t_max && t > t_min {
                    let point = ray.point(t);
                    Some(HitRecord {
                        t,
                        point,
                        normal: (point - center) / radius,
                        material: self.material.as_ref(),
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

pub struct HittableList<H> {
    hittables: Vec<H>,
}

impl<H> HittableList<H> {
    pub fn new(hittables: Vec<H>) -> Self {
        Self { hittables }
    }
}

impl<H> Hittable for HittableList<H>
where
    H: Hittable,
{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            if let Some(temp_rec) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}
