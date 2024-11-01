use crate::prelude::*;

use std::{cmp::max, fs::File, io::{BufWriter, Write}};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vec3::{cross, random_in_unit_disk};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64, 
        image_width: u32, 
        samples_per_pixel: u32,
        max_depth: i32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    
    ) -> Self {
        let image_height = max(1, (image_width as f64 / aspect_ratio) as u32);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = look_from;
        
        // Determine viewport dimensions
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center 
                - focus_dist * w 
                - viewport_u / 2.0 
                - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = 
            focus_dist * f64::tan((defocus_angle / 2.0).to_radians());
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
                          + ((i as f64 + offset.x) * self.pixel_delta_u)
                          + ((j as f64 + offset.y) * self.pixel_delta_v);

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray { origin, direction  }
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn sample_square() -> Vec3 {
        Vec3 {
            x: random::<f64>() - 0.5,
            y: random::<f64>() - 0.5,
            z: 0.0,
        }
    }

    pub fn ray_color(r: &Ray, max_depth: i32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if max_depth <= 0 { return Color::zero(); }

        if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
            if let Some(scatter_rec) = rec.mat.scatter(r, &rec) {
                return scatter_rec.attenuation 
                    * Self::ray_color(&scatter_rec.scattered, max_depth - 1, world);
            }
            return Color::zero();
        }
    
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color { x: 1.0, y: 1.0, z: 1.0 } + a * Color { x: 0.5, y: 0.7, z: 1.0 }
    }

    pub fn render(
        &self, 
        world: &dyn Hittable, 
        writer: &mut BufWriter<File>,
    ) -> std::io::Result<()> 
    {
        let bar = ProgressBar::new(self.image_height as u64);
        writeln!(*writer, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let image: Vec<Color> = (0..self.image_height).into_par_iter()
            .flat_map(|y| {
                bar.inc(1);
                (0..self.image_width)
                    .map(|x| {
                        (0..self.samples_per_pixel).map(|_| {
                            let r = self.get_ray(x, y);
                            Self::ray_color(&r, self.max_depth, world)
                        }).sum()
                    })
                    .collect::<Vec<Color>>()
            }).collect();


        for pixel in image {
            write_color(writer, &(self.pixel_samples_scale * pixel))?;
        }
    
        writer.flush()?;
        bar.finish();
        Ok(())
    }
    
}