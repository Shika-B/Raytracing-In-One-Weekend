use crate::{material::Material, primitives::*};

pub trait Hittable<> {
    fn hit<'a>(&self, ray: &'a Ray, valid_time: Interval) -> Option<(Point3, Vec3, f64, Material, bool)>;
}

#[derive(Default)]
pub struct HittableSequence<'a> {
    pub hittables: Vec<&'a mut dyn Hittable>,
}

impl<'a> HittableSequence<'a> {
    pub fn add(&mut self, obj: &'a mut dyn Hittable) {
        self.hittables.push(obj)
    }
}

impl<'a> Hittable for HittableSequence<'a> {
    fn hit<'b>(&self, ray: &'b Ray, valid_time: Interval) -> Option<(Point3, Vec3, f64, Material, bool)> {
        let mut hit_data: Option<(Point3, Vec3, f64, Material, bool)> = None;

        for obj in self.hittables.iter() {
            if let Some((inter, normal, time, material, face)) = obj.hit(ray, valid_time) {
                unsafe {
                    if hit_data.is_none() || hit_data.unwrap_unchecked().2 > time {
                        hit_data = Some((inter, normal, time, material, face))
                    }
                }
            }
        }
        hit_data
    }
}

impl Hittable for () {
    fn hit<'a>(&self, _: &'a Ray, _: Interval) -> Option<(Point3, Vec3, f64, Material, bool)> {
        None
    }
}
