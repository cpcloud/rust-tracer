#[macro_export]
macro_rules! vec3 {
    [$x:expr, $y:expr, $z:expr] => {
        $crate::Vec3::from([f64::from($x), f64::from($y), f64::from($z)])
    }
}

mod camera;
pub use camera::Camera;

mod material;
pub use material::{Dielectric, Lambertian, Material, Metal};

mod shape;
pub use shape::{Hittable, HittableList, Sphere};

mod colorvec3;
pub use colorvec3::ColorVec3;

mod vec3;
pub use vec3::Vec3;

pub struct HitRecord<'mat> {
    pub t: f64,
    pub point: crate::Vec3,
    pub normal: crate::Vec3,
    pub material: &'mat dyn crate::Material,
}

pub mod utils {
    pub fn rand() -> f64 {
        rand::random()
    }

    pub fn randvec() -> crate::Vec3 {
        [rand(), rand(), rand()].into()
    }
}

mod ray {
    use crate::Vec3;

    #[derive(Debug, Copy, Clone)]
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
        let origin = vec3![1, -2, -3];
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
}

pub use ray::Ray;
