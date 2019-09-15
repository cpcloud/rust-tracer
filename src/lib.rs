#![feature(const_fn, box_syntax)]
#[macro_use]

pub mod vec3;

pub mod material;

pub use material as mat;

pub mod camera;
pub mod hitrecord;
pub mod ray;
pub mod shape;

pub mod utils;
