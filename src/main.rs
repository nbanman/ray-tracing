pub mod prelude;
pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod interval;
pub mod camera;
pub mod material;

use std::{fs::File, io::BufWriter, sync::Arc};

use crate::prelude::*;

use clap::Parser;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output file
    #[arg(short, long, default_value_t = String::from("image.ppm"))]
    output: String,
}

fn main() -> std::io::Result<()> {
    
    let args = Args::parse();
    let file = File::create(&args.output)?;
    let mut writer = BufWriter::new(file);

    // World

    let mut world = HittableList::new();

    let ground_material = 
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.objects.push(
        Box::new(Sphere::stationary(
            Point3::new(0.0, -1000.0, 0.0), 
            1000.0, 
            ground_material
        ))
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Point3::new(
                 a as f64 + 0.9 * random::<f64>(),
                 0.2,
                 b as f64 + 0.9 * random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = 
                        Arc::new(Lambertian::new(albedo));
                    let center2 = center 
                        + Vec3::new(0.0, random_range(0.0, 0.5), 0.0);
                    world.objects.push(Box::new(
                        Sphere::moving(center, center2, 0.2, sphere_material)
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.objects.push(Box::new(
                        Sphere::stationary(center, 0.2, sphere_material)
                    ));
                } else {
                    // glass
                    let sphere_material = 
                        Arc::new(Dielectric::new(1.5));
                    world.objects.push(Box::new(
                        Sphere::stationary(center, 0.2, sphere_material)
                    ));
                }

            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.objects.push(Box::new(
        Sphere::stationary(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)
    ));

    let mat2 = 
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.objects.push(Box::new(
        Sphere::stationary(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)
    ));

    let mat3 = 
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    world.objects.push(Box::new(
        Sphere::stationary(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)
    ));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0
    );

    cam.render(&world, &mut writer)?;
    
    Ok(())
}

