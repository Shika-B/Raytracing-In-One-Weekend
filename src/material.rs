use crate::primitives::*;

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Material {
    DiffuseBasic(Color),
    Lambertian(Color),
    Metallic{
        color: Color,
        fuzz: f64
    },
}

impl Material {
    #[allow(unused)]
    pub fn scatter(&self, ray: &Ray, normal: Vec3, point: Point3, face: bool) -> Option<(Color, Ray)> {
        match self {
            Self::DiffuseBasic(color) => {
                let mut new_dir = vec3_random_hemisphere(&normal);
                if new_dir.norm_squared() < 1e-16 {
                    new_dir = normal;
                }
                return Some((*color, Ray::new(point, new_dir)));
            }
            Self::Lambertian(color) => {
                let mut new_dir = normal + vec3_random_unit();
                if new_dir.norm_squared() < 1e-16 {
                    new_dir = normal;
                }
                return Some((*color, Ray::new(point, new_dir)));
            },
            Self::Metallic { color, fuzz } => {
                let new_dir = ray.dir - 2.0*ray.dir.dot(&normal)*normal;
                let normal_new_dir = new_dir.normalize();
                let fuzzed_dir = normal_new_dir + *fuzz*vec3_random_unit();
                if fuzzed_dir.dot(&normal) < 0.0 {
                    return None
                }
                return Some((*color, Ray::new(point, new_dir)))
            }
            _ => unimplemented!(),
        }
    }
}
