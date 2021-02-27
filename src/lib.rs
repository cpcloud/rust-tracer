#![feature(const_fn)]
#[macro_use]

pub mod vec3;

#[macro_export]
macro_rules! vec3 {
    [$x:expr, $y:expr, $z:expr] => {
        $crate::vec3::Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    }
}

pub mod material;
pub use material as mat;

pub mod camera;
pub mod hitrecord;
pub mod ray;
pub mod shape;

extern crate rand;
pub mod utils;
