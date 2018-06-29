use hitrecord::HitRecord;
use hittable::Hittable;
use material::Material;
use ray::Ray;
use std::f64;
use std::fmt;
use vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let radius = self.radius;
        let center = self.center;

        let oc = ray.origin() - center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - radius.powi(2);
        let disc = b.powi(2) - a * c;
        if disc > 0.0 {
            let temp = (-b - disc.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point(temp);
                Some(HitRecord::new(temp, p, (p - center) / radius, &self.mat))
            } else {
                let temp = (-b + disc.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let p = ray.point(temp);
                    Some(HitRecord::new(temp, p, (p - center) / radius, &self.mat))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

pub fn sphere(center: Vec3, radius: f64, mat: Box<Material>) -> Box<Hittable> {
    box Sphere::new(center, radius, mat)
}

struct HittableList {
    items: Vec<Box<Hittable>>,
}

impl fmt::Debug for HittableList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HittableList")
    }
}

impl HittableList {
    pub fn new(items: Vec<Box<Hittable>>) -> Self {
        HittableList { items }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for item in self.items.iter() {
            item.hit(ray, t_min, closest_so_far).map(|temp_rec| {
                closest_so_far = temp_rec.t();
                rec = Some(temp_rec);
            });
            //            if let Some(temp_rec) =  {
            //            }
        }
        rec
    }
}

pub fn hittable_list(items: Vec<Box<Hittable>>) -> Box<Hittable> {
    box HittableList::new(items)
}
