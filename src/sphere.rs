use std::sync::Arc;

use crate::prelude::*;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn stationary(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let center = Ray::new(static_center, Vec3::zero(), 0.0);
        Sphere { center, radius, mat, }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.0);
        Sphere { center, radius, mat, }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self, 
        r: &crate::ray::Ray, 
        ray_t: Interval, 
    ) -> Option<HitRecord> 
    {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.origin;
        let a = r.direction.len_squared();
        let h = dot(r.direction, oc);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
    
        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) { return None;}
        }

        let mut rec = HitRecord {
            p: r.at(root),
            normal: Default::default(),
            mat: self.mat.clone(),
            t: root,
            front_face: Default::default(),
        };
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}