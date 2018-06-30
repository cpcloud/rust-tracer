use super::mat::Material;
use vec3::Vec3;

pub struct HitRecord<'a> {
    t: f64,
    p: Vec3,
    normal: Vec3,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub const fn new(t: f64, p: Vec3, normal: Vec3, material: &'a Material) -> Self {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn point(&self) -> Vec3 {
        self.p
    }

    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    pub const fn material(&self) -> &'a Material {
        self.material
    }
}
