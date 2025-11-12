mod camera;
mod hittable;
mod image;
mod material;
mod primitives;
mod sphere;

use camera::Camera;
use hittable::HittableSequence;
use image::Image;
use primitives::*;
use sphere::Sphere;

use crate::material::Material;

fn write_image_scene() {
        let sample_per_pixels = 128;
    let max_depth = 5;

    let width = 800;
    let ratio = 16.0 / 9.0;
    let vfov = 90.0;

    //let lookfrom = Point3::new(-2.0, 2.0, 1.0);
    let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Point3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        ratio,
        width,
        vfov,
        sample_per_pixels,
        max_depth,
    );
    let (height, width) = (camera.height, camera.width);
    let mut image = Image::new_dark(height, width);

    let mut world = HittableSequence::default();
    let diffuse1 = Material::Lambertian(Color::new(0.2, 0.8, 0.4));
    let diffuse2 = Material::Lambertian(Color::new(1.0, 0.0, 0.0));
    let diffuse3 = Material::Lambertian(Color::new(0.0, 1.0, 0.0));
    let diffuse4 = Material::Metallic {
        fuzz: 0.125,
        color: Color::new(0.8, 0.8, 0.8),
    };

    // let r = f64::consts::FRAC_PI_4.cos();
    let mut sphere1 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, diffuse1);
    let mut sphere2 = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, diffuse2);
    let mut sphere3 = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, diffuse3);
    let mut sphere4 = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, diffuse4);

    world.add(&mut sphere1);
    world.add(&mut sphere2);
    world.add(&mut sphere3);
    world.add(&mut sphere4);

    camera.render(&world, &mut image);

    println!("Rendering finished");
    image.write_to("output.ppm");
}

fn main() {
    write_image_scene();
}
