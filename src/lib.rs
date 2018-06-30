#![feature(const_fn)]
#![feature(box_syntax)]
#[macro_use]

pub mod vec3;

pub mod camera;
pub mod hitrecord;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod shape;

extern crate rand;
pub mod utils;
