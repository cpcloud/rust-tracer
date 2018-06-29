use hitrecord::HitRecord;
use ray::Ray;
use std::marker::Sync;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
