use std::{
    iter::{FromIterator, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub},
};

pub trait GeomVec {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

pub trait ColorVec {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub const ZEROS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const ONES: Vec3 = Vec3::new(1.0, 1.0, 1.0);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub const fn len(&self) -> usize {
        3usize
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub const fn zeros() -> Self {
        ZEROS
    }

    pub const fn ones() -> Self {
        ONES
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    pub fn unitize(&self) -> Vec3 {
        *self / self.norm()
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn dot<T: AsRef<Vec3>>(&self, other: T) -> f64 {
        let Vec3 { x, y, z } = other.as_ref();
        self.x * x + self.y * y + self.z * z
    }

    pub fn cross<T: AsRef<Vec3>>(&self, other: T) -> Vec3 {
        let Vec3 { x, y, z } = other.as_ref();
        Vec3 {
            x: self.y * z - self.z * y,
            y: -(self.x * z - self.z * x),
            z: self.x * y - self.y * x,
        }
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn powf(&self, n: f64) -> Vec3 {
        Vec3 {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
        }
    }

    pub fn lerp(&self, b: Vec3, t: f64) -> Vec3 {
        (1.0 - t) * *self + t * b
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
        let uv = self.unitize();
        let dt = uv.dot(n);
        let disc = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
        if disc > 0.0 {
            Some(ni_over_nt * (uv - n * dt) - n * disc.sqrt())
        } else {
            None
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        iter.fold(Vec3::zeros(), Add::add)
    }
}

impl FromIterator<f64> for Vec3 {
    fn from_iter<I: IntoIterator<Item = f64>>(it: I) -> Self {
        let mut iter = it.into_iter();
        let x = iter
            .next()
            .expect("Iterator must have exactly 3 elements, found 0");
        let y = iter
            .next()
            .expect("Iterator must have exactly 3 elements, found 1");
        let z = iter
            .next()
            .expect("Iterator must have exactly 3 elements, found 2");
        match iter.next() {
            None => Vec3 { x, y, z },
            _ => panic!("Converting to Vec3 from container with more than 3 elements"),
        }
    }
}

//#[test]
//fn test_from_vec() {
//    let vec = vec![1.0, 2.0, 3.0];
//    let result: Vec3 = vec.iter().collect();
//    let expected = vec3![1, 2, 3];
//    assert_eq!(result, expected);
//}

impl AsRef<Vec3> for Vec3 {
    fn as_ref(&self) -> &Vec3 {
        self
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        let Vec3 { x, y, z } = self;
        match i {
            0 => &x,
            1 => &y,
            2 => &z,
            _ => panic!("Invalid index {}, must be 0, 1, or 2", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index {}, must be 0, 1, or 2", i),
        }
    }
}

#[test]
fn test_new() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(
        u,
        Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        }
    );
}

#[test]
fn test_len() {
    let u = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(u.len(), 3);
}

#[test]
fn test_is_empty() {
    let u = crate::vec3![0, 0, 0];
    assert_eq!(u.is_empty(), false);
}

#[test]
fn test_norm2() {
    let expected = 14.0;
    let u = Vec3::new(1.0, 2.0, 3.0);
    let result = u.norm2();
    assert_eq!(result, expected);
}

#[test]
fn test_norm() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let result = u.norm();
    let expected = (1.0 + 4.0 + 9.0f64).sqrt();
    assert_eq!(result, expected);
}

#[test]
fn test_unitize() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let result = u.unitize();
    let expected = u / u.norm();
    assert_eq!(result, expected);
}

#[test]
fn test_dot() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = u * 2.0;
    let result = u.dot(v);
    let expected = 1.0 * 2.0 + 2.0 * 4.0 + 3.0 * 6.0;
    assert_eq!(result, expected);
}

#[test]
fn test_cross() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = u * 2.0;
    let result = u.cross(v);
    let expected = Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: -(u.x * v.z - u.z * v.x),
        z: u.x * v.y - u.y * v.x,
    };
    assert_eq!(result, expected);
}

impl GeomVec for Vec3 {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
}

#[test]
fn test_geom_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(u.x(), 1.0);
    assert_eq!(u.y(), 2.0);
    assert_eq!(u.z(), 3.0);
}

impl ColorVec for Vec3 {
    fn r(&self) -> u8 {
        (255.0 * self.x) as u8
    }
    fn g(&self) -> u8 {
        (255.0 * self.y) as u8
    }
    fn b(&self) -> u8 {
        (255.0 * self.z) as u8
    }
}

#[test]
fn test_color_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(u.r(), 255);
    assert_eq!(u.g(), 255 * 2);
    assert_eq!(u.b(), 255 * 3);
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x + other,
            y: y + other,
            z: z + other,
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = other;
        Vec3 {
            x: self + x,
            y: self + y,
            z: self + z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x + other.x,
            y: y + other.y,
            z: z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x - other,
            y: y - other,
            z: z - other,
        }
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = other;
        Vec3 {
            x: self - x,
            y: self - y,
            z: self - z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x - other.x,
            y: y - other.y,
            z: z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x * other,
            y: y * other,
            z: z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = other;
        Vec3 {
            x: self * x,
            y: self * y,
            z: self * z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x * other.x,
            y: y * other.y,
            z: z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x / other,
            y: y / other,
            z: z / other,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = other;
        Vec3 {
            x: self / x,
            y: self / y,
            z: self / z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: x / other.x,
            y: y / other.y,
            z: z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

#[test]
fn test_add_vec_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = Vec3::new(2.0, -2.0, 5.0);
    let z = u + v;
    assert_eq!(z, Vec3::new(3.0, 0.0, 8.0));
}

#[test]
fn test_add_vec_scalar() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = -1.0;
    let z = u + v;
    assert_eq!(z, Vec3::new(0.0, 1.0, 2.0));
}

#[test]
fn test_add_scalar_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = 1.0;
    let z = v + u;
    assert_eq!(z, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_sub_vec_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = Vec3::new(2.0, -2.0, 5.0);
    let z = u - v;
    assert_eq!(z, Vec3::new(-1.0, 4.0, -2.0));
}

#[test]
fn test_sub_vec_scalar() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = -1.0;
    let z = u - v;
    assert_eq!(z, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_sub_scalar_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = 1.0;
    let z = v - u;
    assert_eq!(z, Vec3::new(0.0, -1.0, -2.0));
}

#[test]
fn test_mul_vec_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = Vec3::new(2.0, -2.0, 5.0);
    let z = u * v;
    assert_eq!(z, Vec3::new(2.0, -4.0, 15.0));
}

#[test]
fn test_mul_vec_scalar() {
    let u = Vec3::new(1.0, 2.0, -3.0);
    let v = -30.0;
    let z = u * v;
    assert_eq!(z, Vec3::new(-30.0, -60.0, 90.0));
}

#[test]
fn test_mul_scalar_vec() {
    let u = Vec3::new(1.0, 2.0, -3.0);
    let v = -30.0;
    let z = v * u;
    assert_eq!(z, Vec3::new(-30.0, -60.0, 90.0));
}

#[test]
fn test_div_vec_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = Vec3::new(2.0, -2.0, 5.0);
    let z = u / v;
    assert_eq!(z, Vec3::new(0.5, -1.0, 3.0 / 5.0));
}

#[test]
fn test_div_vec_scalar() {
    let u = Vec3::new(1.0, 2.0, -3.0);
    let v = -30.0;
    let z = u / v;
    assert_eq!(z, Vec3::new(1.0 / -30.0, 2.0 / -30.0, -3.0 / -30.0));
}

#[test]
fn test_div_scalar_vec() {
    let u = Vec3::new(1.0, 2.0, -3.0);
    let v = -30.0;
    let z = v / u;
    assert_eq!(z, Vec3::new(-30.0 / 1.0, -30.0 / 2.0, -30.0 / -3.0));
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}

#[test]
fn test_neg_vec() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let result = -u;
    let expected = Vec3::new(-1.0, -2.0, -3.0);
    assert_eq!(result, expected);
}
