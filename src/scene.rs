
use crate::ray::Ray;
use crate::image::Color;
use crate::hittables::Hittable;
use crate::hittables::HitRecord;

pub struct Scene {
    tmin:f32,
    tmax:f32,
    contents: Vec<Box<dyn Hittable>>,
}

use cgmath::InnerSpace;

pub fn new() -> Scene {
    Scene {
        tmin: 0.001,
        tmax: 1000.0,
        contents: Vec::new(),
    }
}

impl Scene {
    pub fn cast(&self, r:&Ray, depth:usize) -> Color {
        if depth == 0 {
            return self.on_miss(r);
        }
        match self.contents.hit(r, self.tmin, self.tmax) {
            None => self.on_miss(r),
            Some(hit) => self.on_hit(r, depth, hit),
        }
    }

    pub fn on_miss(&self, ray:&Ray) -> Color {
        let dir = ray.direction().normalize();
        let t = 0.5 * (dir.y + 1.0);
        Color::lerp(t,
            Color{red:1.0, green:1.0, blue:1.0},
            Color{red:0.5, green:0.7, blue:1.0},
        )
    }
    
    pub fn on_hit(&self, ray:&Ray, depth:usize, hit:HitRecord) -> Color {
        match hit.material.scatter(&ray, &hit) {
            None => Color::black(),
            Some(scatter) => {
                scatter.attenuation * self.cast(&scatter.ray, depth-1)
            }
        }
    }

    pub fn push(&mut self, hittable:Box<dyn Hittable>) {
        self.contents.push(hittable);
    }
}
