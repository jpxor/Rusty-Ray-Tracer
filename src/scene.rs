
use crate::ray::Ray;
use crate::hittables::Hittable;
use crate::hittables::HitRecord;

pub struct Scene {
    contents: Vec<Box<dyn Hittable>>,
}

impl Scene {

    pub fn new() -> Self {
        Scene { contents: Vec::new() }
    }

    pub fn hit(&self, r:&Ray, tmin:f32, tmax:f32) -> Option<HitRecord> {
        self.contents.hit(r, tmin, tmax)
    }

    pub fn push(&mut self, hittable:Box<dyn Hittable>) {
        self.contents.push(hittable);
    }

}
