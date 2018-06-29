use material::Material;
use vec3::Vec3;

pub struct HitRecord<'a> {
    t: f64,
    p: Vec3,
    normal: Vec3,
    pub mat: &'a Box<Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Vec3, normal: Vec3, mat: &'a Box<Material>) -> Self {
        HitRecord { t, p, normal, mat }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn point(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}
