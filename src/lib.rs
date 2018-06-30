#![feature(const_fn)]
#![feature(box_syntax)]
#[macro_use]

pub mod vec3;

pub mod matenum;
pub mod material;

pub use material as mat;
//pub use matenum as mat;

pub mod camera;
pub mod hitrecord;
pub mod ray;
pub mod shape;

extern crate rand;
pub mod utils;
