use vec3::{random_unit_vector, reflect, refract};

use crate::prelude::*;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
            &self,
            _r_in: &Ray,
            rec: &HitRecord,
        ) -> Option<ScatterRecord> 
    {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = f64::min(1.0, fuzz);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
        ) -> Option<ScatterRecord> 
    {
        let reflected = reflect(r_in.direction.unit_vector(), rec.normal);
        let reflected = reflected.unit_vector() + (self.fuzz * random_unit_vector());
        
        let scattered = Ray::new(rec.p, reflected);

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
        ) -> Option<ScatterRecord> {
        let ri = if rec.front_face { 
            1.0 / self.refraction_index 
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract 
            || Self::reflectance(cos_theta, ri) > random()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };

        Some(ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p, direction),
        })
    }
}
