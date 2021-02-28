#[macro_export]
macro_rules! vec3 {
    [$x:expr, $y:expr, $z:expr] => {
        $crate::Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    }
}

mod camera;
pub use camera::Camera;

mod material;
pub use material::{Dielectric, Lambertian, Material, Metal};

mod ray;
pub use ray::Ray;

mod shape;
pub use shape::{Hittable, HittableList, Sphere};

pub mod utils;

mod vec3;
pub use vec3::{ColorVec, GeomVec, Vec3};

pub struct HitRecord<'mat> {
    pub t: f64,
    pub point: crate::Vec3,
    pub normal: crate::Vec3,
    pub material: &'mat dyn crate::Material,
}
