use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point(&self, t: f64) -> Vec3 {
        self.origin() + t * self.direction()
    }
}

#[test]
fn test_ray() {
    let origin = v3![1.0, -2.0, -3.0];
    let direction = origin * 1.03;
    let result = Ray::new(origin, direction);
    assert_eq!(result.origin(), origin);
    assert_eq!(result.direction(), direction);
    assert_eq!(result.point(-2.0), origin - (direction + direction));
    assert_eq!(result.point(-1.0), origin - direction);
    assert_eq!(result.point(0.0), origin);
    assert_eq!(result.point(1.0), origin + direction);
    assert_eq!(result.point(2.0), origin + 2.0 * direction);
}
