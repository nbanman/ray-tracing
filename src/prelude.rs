pub use crate::{ray, vec3};
pub use crate::vec3::Color;
pub use crate::interval::Interval;
pub use crate::color::write_color;
use rand::Rng;
pub use vec3::{dot, Point3, Vec3};
pub use ray::Ray;
pub use crate::hittable::{HitRecord, Hittable};
pub use crate::hittable_list::HittableList;
pub use crate::camera::Camera;
pub use crate::material::Material;


pub use std::rc::Rc;
pub use std::f64::INFINITY;

pub use rand::random;

pub fn random_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}