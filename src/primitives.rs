use core::f64;

use nalgebra::{Vector2, Vector3};

pub type Point3 = Vector3<f64>;
pub type Vec3 = Vector3<f64>;

pub fn deg_to_rad(x: f64) -> f64 {
    x * f64::consts::PI / 180.0
}
pub fn vec3_random_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        rand::random_range(min..max),
        rand::random_range(min..max),
        rand::random_range(min..max),
    )
}
pub fn vec3_random_unit() -> Vec3 {
    loop {
        let v = vec3_random_range(-1.0, 1.0);
        if v.norm_squared() >= 1e-160 && v.norm_squared() < 1.0 {
            return v.normalize();
        }
    }
}
#[allow(unused)]
pub fn vec2_random_unit() -> Vector2<f64> {
    loop {
        let v = Vector2::new(
            rand::random_range(-1.0..1.0),
            rand::random_range(-1.0..1.0),
        );
        if v.norm_squared() >= 1e-160 && v.norm_squared() < 1.0 {
            return v.normalize();
        }
    }
}

pub fn vec3_random_hemisphere(normal: &Vec3) -> Vec3 {
    let v = vec3_random_unit();
    if v.dot(normal) < 0.0 {
        -v
    } else {
        v
    }
}

pub type Interval = (f64, f64);
#[allow(unused)]
pub const POSITIVES: Interval = (0.0, f64::INFINITY);
pub const STRICT_POSITIVES: Interval = (0.001, f64::INFINITY);

pub type Color = Vector3<f64>;

#[allow(unused)]
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self { origin, dir }
    }
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.dir;
    }
}
