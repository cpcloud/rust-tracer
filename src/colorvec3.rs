use crate::Vec3;
use nalgebra as na;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ColorVec3(na::Vector3<u8>);

impl ColorVec3 {
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn into_array(self) -> [u8; 3] {
        self.0.into()
    }
}

impl From<Vec3> for ColorVec3 {
    fn from(vec: Vec3) -> Self {
        Self(
            (vec * f64::from(u8::MAX))
                .into_inner()
                .map(|value| value as u8),
        )
    }
}

impl From<na::Vector3<f64>> for ColorVec3 {
    fn from(vec: na::Vector3<f64>) -> Self {
        Vec3::from(vec).into()
    }
}

impl From<na::Vector3<u8>> for ColorVec3 {
    fn from(vec: na::Vector3<u8>) -> Self {
        Self(vec)
    }
}

impl From<[f64; 3]> for ColorVec3 {
    fn from(vec: [f64; 3]) -> Self {
        Vec3::from(vec).into()
    }
}

#[cfg(test)]
mod tests {
    use super::ColorVec3;
    use crate::Vec3;

    #[test]
    fn test_color_vec() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        let u = ColorVec3::from(v);
        assert_eq!(u.r(), 255);
        assert_eq!(u.g(), (255.0 * 2.0) as u8);
        assert_eq!(u.b(), (255.0 * 3.0) as u8);
    }
}
