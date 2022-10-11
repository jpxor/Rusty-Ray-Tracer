
use std::sync::Arc;

use crate::ray::Ray;
use crate::materials::Material;

type Vector3 = cgmath::Vector3<f32>;

pub trait Hittable: Sync+Send {
    fn hit(&self, ray:&Ray, tmin:f32, tmax:f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {

    pub fn new(t:f32, point:Vector3, normal:Vector3, ray:&Ray, material:Arc<dyn Material>) -> HitRecord {
        let mut rec = HitRecord{
            t,
            point,
            normal,
            material,
            front_face: false,
        };
        rec.set_face_normal(ray, normal);
        rec
    }

    fn set_face_normal(&mut self, ray:&Ray, outward_normal:Vector3) {
        self.front_face = cgmath::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }

}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray:&Ray, tmin:f32, tmax:f32) -> Option<HitRecord> {
        let mut result:Option<HitRecord> = None;
        let mut closest = tmax;
        for hittable in self {
            match hittable.hit(ray, tmin, closest) {
                None => continue,
                Some(hit_result) => {
                    closest = hit_result.t;
                    result = Some(hit_result);
                }
            }
        }
        result
    }
}

pub struct Sphere {
    pub material: Arc<dyn Material>,
    pub origin: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(origin:Vector3, radius:f32, material: Arc<dyn Material>) -> Box<Sphere> {
        Box::new( Sphere{ material, origin, radius } )
    }
}

impl Hittable for Sphere {

    fn hit(&self, ray:&Ray, tmin:f32, tmax:f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.origin;
        let a = cgmath::dot(ray.direction(), ray.direction());
        let half_b = cgmath::dot(oc, ray.direction());
        let c = cgmath::dot(oc, oc) - self.radius * self.radius;
        let d = half_b*half_b - a*c;

        if d < 0.0 {
            return None;
        }
        let sqrt_d = d.sqrt();
        let inv_a = 1.0 / a;
        let left_root = (-half_b - sqrt_d) * inv_a;

        let t = match in_range(left_root, tmin, tmax) {
            true => left_root,
            false => {
                let right_root = (-half_b + sqrt_d) * inv_a;
                match in_range(right_root, tmin, tmax) {
                    true => right_root,
                    false => {
                        return None;
                    }
                }
            }
        };
        let point = ray.at(t);
        let normal = (point - self.origin) / self.radius;
        Some(HitRecord::new(t, point, normal, ray, self.material.clone()))
    }
}

fn in_range(v:f32, min:f32, max:f32) -> bool {
    min <= v && v <= max
}
