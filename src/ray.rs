
type Vector3 = cgmath::Vector3<f32>;
use cgmath::InnerSpace;

pub struct Ray {
   origin: Vector3,
   direction: Vector3,
}

impl Ray {
    pub fn new(origin:Vector3, direction:Vector3) -> Ray {
        let direction = direction.normalize();
        Ray {origin, direction}
    }

    #[inline]
    pub fn at(&self, t:f32) -> Vector3 {
        self.origin + t*self.direction
    }

    #[inline]
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}