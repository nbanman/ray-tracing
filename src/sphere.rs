use std::sync::Arc;

use crate::prelude::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
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
        let oc = self.center - r.origin;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}