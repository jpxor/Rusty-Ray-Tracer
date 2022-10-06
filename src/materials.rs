
use std::sync::Arc;
use rand::Rng;

use crate::ray::Ray;
use crate::image::Color;
use crate::hittables::HitRecord;

type Vector3 = cgmath::Vector3<f32>;
use cgmath::AbsDiffEq;

use crate::renderer::random_unit_vector3;

pub struct Scattered {
    pub attenuation:Color,
    pub ray:Ray,
}

impl Scattered {
    pub fn new(ray:Ray, attenuation:Color) -> Option<Scattered> {
        Some(Scattered{
            attenuation,
            ray,
        })
    }
}

pub trait Material: Sync+Send {
    fn scatter(&self, ray:&Ray, hit:&HitRecord) -> Option<Scattered>;
}

pub fn equal(a:&Vector3, b:&Vector3) -> bool {
    let epsilon = cgmath::Vector3::<f32>::default_epsilon();
    a.abs_diff_eq(b, epsilon)
}

pub fn reflect(v:Vector3, n:Vector3) -> Vector3 {
    v - 2.0 * cgmath::dot(v, n) * n
}

pub fn refract(uv:Vector3, n:Vector3, ratio:f32) -> Vector3 {
    let cos_theta = f32::min(cgmath::dot(-uv,n), 1.0);
    let perpendicular = ratio * (uv + cos_theta * n);
    let parallel = -f32::abs(1.0 - cgmath::dot(perpendicular,perpendicular)).sqrt() * n;
    perpendicular + parallel
}

pub struct Lambertian {
    pub albedo:Color,
}

impl Lambertian {
    pub fn new(albedo:Color) -> Arc<Lambertian> {
        Arc::new(Lambertian{albedo})
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray:&Ray, hit:&HitRecord) -> Option<Scattered> {
        let perturbation = random_unit_vector3();
        let scatter_dir = match equal(&hit.normal, &-perturbation) {
            false => hit.normal + perturbation,
            true  => hit.normal,
        };
        Scattered::new(
            Ray::new(hit.point, scatter_dir), 
            self.albedo,
        )
    }
}

pub struct Metal {
    pub albedo:Color,
    pub roughness:f32,
}

impl Metal {
    pub fn new(albedo:Color, roughness:f32) -> Arc<Metal> {
        let roughness = roughness.clamp(0.0, 1.0);
        Arc::new(Metal{albedo, roughness})
    }
}

impl Material for Metal {
    fn scatter(&self, ray:&Ray, hit:&HitRecord) -> Option<Scattered> {
        let reflection = reflect(ray.direction(), hit.normal);
        let perturbation = random_unit_vector3();

        let reflection = match equal(&reflection, &-perturbation) {
            false => reflection + self.roughness * perturbation,
            true  => reflection,
        };
        Scattered::new(
            Ray::new(hit.point, reflection), 
            self.albedo,
        )
    }
}

pub struct Dialectric {
    pub refraction_index:f32,
}

impl Dialectric {
    pub fn new(refraction_index:f32) -> Arc<Dialectric> {
        Arc::new(Dialectric{refraction_index})
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray:&Ray, hit:&HitRecord) -> Option<Scattered> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match hit.front_face {
            true => 1.0/self.refraction_index,
            false => self.refraction_index,
        };
        let cos_theta = f32::min(cgmath::dot(-ray.direction(), hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let reflectance = |cosine:f32, index:f32| {
            let r0 = (1.0-index) / (1.0+index);
            let r0 = r0*r0;
            r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
        };
        let chance_reflect = || {
            let mut rng = rand::thread_rng();
            reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        };
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        
        let direction = match cannot_refract || chance_reflect() {
            true  => reflect(ray.direction(), hit.normal),
            false => refract(ray.direction(), hit.normal, refraction_ratio),
        };
        Scattered::new(
            Ray::new(hit.point, direction), 
            attenuation,
        )
    }
}
