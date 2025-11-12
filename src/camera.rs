use crate::{hittable::Hittable, image::Image, primitives::*};

use rand::Rng;

pub struct Camera {
    pub width: usize,
    pub height: usize,
    pub sample_per_pixels: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub camera_frame: (Vec3, Vec3, Vec3),
    pix_delta_x: Vec3,
    pix_delta_y: Vec3,
    pixel_up_left: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        up: Vec3,
        ratio: f64,
        width: usize,
        vfov: f64,
        sample_per_pixels: usize,
        max_depth: usize,
    ) -> Self {
        let height = (width as f64 / ratio).round() as usize;
        let focal_length = (lookfrom - lookat).norm();
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        let w = (lookfrom - lookat).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_x = (viewport_width as f64) * u;
        let viewport_y = (-viewport_height as f64) * v;

        let pix_delta_x = viewport_x / width as f64;
        let pix_delta_y = viewport_y / height as f64;

        let viewport_up_left = lookfrom - focal_length * w - viewport_x / 2.0 - viewport_y / 2.0;
        let pixel_up_left = viewport_up_left + 0.5 * (pix_delta_x + pix_delta_y);

        Self {
            lookfrom,
            lookat,
            width,
            height,
            vfov,
            sample_per_pixels,
            max_depth,
            pix_delta_x,
            pix_delta_y,
            pixel_up_left,
            camera_frame: (u, v, w),
        }
    }

    pub fn ray_color<'a, T: Hittable>(&self, world: &T, ray: &'a Ray, depth: usize) -> Color {
        if depth == 0 {
            return BLACK;
        }

        if let Some((inter, normal, _t, material, face)) = world.hit(ray, STRICT_POSITIVES) {
            if let Some((color, scattered)) = material.scatter(ray, normal, inter, face) {
                return color.component_mul(&self.ray_color(world, &scattered, depth - 1));
            }
        }

        let u_dir = ray.dir.normalize();
        let a = 0.5 * (u_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render<'a, T: Hittable>(&self, world: &T, image: &'a mut Image) {
        assert!(self.width == image.width && self.height == image.height);
        let mut percent = 0.0;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut s = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixels {
                    let ray = self.get_ray(x, y);
                    let pix_color = self.ray_color(world, &ray, self.max_depth);
                    s += pix_color;
                }
                let mean_color = s / (self.sample_per_pixels as f64);
                image.set(x, y, mean_color)
            }
            if (y as f64)/(self.height as f64) > percent/100.0 {
                percent += 10.0;
                println!("Finished {}% of the rendering process", percent);
            }
        }
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let (x_eps, y_eps) = (
            rand::rng().random_range(-0.5..0.5),
            rand::rng().random_range(-0.5..0.5),
        );
        let pix = self.pixel_up_left
            + ((x as f64 + x_eps) * self.pix_delta_x)
            + ((y as f64 + y_eps) * self.pix_delta_y);
        let ray_dir = pix - self.lookfrom;
        Ray::new(self.lookfrom, ray_dir)
    }
}
