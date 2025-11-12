use crate::{hittable::Hittable, material::Material, primitives::*};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&self, ray: &'a Ray, valid_time: Interval) -> Option<(Point3, Vec3, f64, Material, bool)> {
        let v = self.center - ray.origin;
        let a = ray.dir.norm_squared();
        let h = ray.dir.dot(&v);
        let c = v.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let sqrt_d = discriminant.sqrt();
            let mut t = (h - sqrt_d) / a;

            if t <= valid_time.0 || valid_time.1 <= t {
                t = (h + sqrt_d) / a;
                if t <= valid_time.0 || t <= valid_time.1 {
                    return None;
                }
            }

            let intersect = ray.at(t);
            let outward_normal = (intersect - self.center).normalize();
            let front_face = ray.dir.dot(&outward_normal) < 0.0;
            let normal = if front_face {
                outward_normal
            } else {
                -outward_normal
            };
            Some((intersect, normal, t, self.material, front_face))
        }
    }
}
