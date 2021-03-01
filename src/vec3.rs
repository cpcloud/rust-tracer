use nalgebra as na;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(na::Vector3<f64>);

impl From<na::Vector3<f64>> for Vec3 {
    fn from(vector3: na::Vector3<f64>) -> Self {
        Self(vector3)
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self(na::Vector3::new(x, y, z))
    }
}

impl From<Vec<f64>> for Vec3 {
    fn from(vec: Vec<f64>) -> Self {
        Self(na::Vector3::from_vec(vec))
    }
}

impl Vec3 {
    pub fn into_array(self) -> [f64; 3] {
        self.0.into()
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn into_inner(self) -> na::Vector3<f64> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn zeros() -> Self {
        [0.0; 3].into()
    }

    pub fn ones() -> Self {
        [1.0; 3].into()
    }

    pub fn norm2(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.0.norm()
    }

    pub fn unitize(&self) -> Self {
        na::Unit::new_normalize(self.0).into_inner().into()
    }

    pub fn sum(&self) -> f64 {
        self.0.sum()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.0.dot(&other.0)
    }

    pub fn cross(&self, other: Self) -> Self {
        self.0.cross(&other.0).into()
    }

    pub fn sqrt(&self) -> Self {
        self.0.map(f64::sqrt).into()
    }

    pub fn powf(&self, n: f64) -> Self {
        self.0.map(|value| f64::powf(value, n)).into()
    }

    pub fn lerp(&self, b: Self, t: f64) -> Self {
        self.0.lerp(&b.0, t).into()
    }

    pub fn reflect(&self, n: impl Into<Self>) -> Self {
        let reflection = na::Reflection::new(na::Unit::new_normalize(n.into().0), 0.0);
        let mut out = *self;
        reflection.reflect(&mut out.0);
        out
    }

    pub fn refract(&self, n: Self, ni_over_nt: f64) -> Option<Self> {
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
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zeros(), Add::add)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        self.0.index(i)
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        self.0.index_mut(i)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_reflect() {
        let vec = Vec3::from([1.0, 0.0, 1.0]);
        let reflected = vec.reflect([0.0, 0.0, 1.0]);
        assert_eq!(reflected, Vec3::from([1.0, 0.0, -1.0]));
    }

    #[test]
    fn test_xyx() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(u.x(), 1.0);
        assert_eq!(u.y(), 2.0);
        assert_eq!(u.z(), 3.0);
    }

    #[test]
    fn test_new() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(u[0], 1.0);
        assert_eq!(u[1], 2.0);
        assert_eq!(u[2], 3.0);
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
        let u = Vec3::from([1.0, 2.0, 3.0]);
        let result = u.unitize();
        let expected = u / u.norm();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dot() {
        let u = Vec3::from([1.0, 2.0, 3.0]);
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
        let expected = Vec3::from([
            u.y() * v.z() - u.z() * v.y(),
            -(u.x() * v.z() - u.z() * v.x()),
            u.x() * v.y() - u.y() * v.x(),
        ]);
        assert_eq!(result, expected);
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Self::from(self.0.add_scalar(other))
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.0.add_scalar_mut(other)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, other: Self::Output) -> Self::Output {
        other + self
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self::from(self.0 + other.0)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self::Output {
        self.0.add_scalar(-other).into()
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: Self::Output) -> Self::Output {
        Self::Output::from([self; 3]) - other
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::from(self.0 - other.0)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::Output::from(self.0 * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Self::Output) -> Self::Output {
        Self::Output::from(self * other.0)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.0.component_mul(&other.0).into()
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self::from(self.0 / other)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Self::Output) -> Self::Output {
        Self::Output::from([self; 3]) / other
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.0.component_div(&other.0).into()
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.0.component_div_assign(&other.0)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Self::from(-self.0)
    }
}

#[cfg(test)]
mod arith_tests {
    use super::Vec3;

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

    #[test]
    fn test_neg_vec() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let result = -u;
        let expected = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(result, expected);
    }
}
