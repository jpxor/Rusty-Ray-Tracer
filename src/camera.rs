
use rand::Rng;
use cgmath::InnerSpace;
use crate::ray::Ray;

type Vector3 = cgmath::Vector3<f32>;

#[derive(Clone, Copy)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,
}

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vector3,
    h_unit: Vector3,
    v_unit: Vector3,
    vp_center: Vector3,
    vp_horizontal: Vector3,
    vp_vertical: Vector3,
    lens_radius:f32,
}

fn random_in_unit_disk() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if cgmath::dot(p, p) >= 1.0 {
            continue
        } else {
            return p;
        }
    }
}

impl Camera {
    pub fn new(origin:Vector3,
               target:Vector3,
               up:Vector3,
               vfov:f32, /* vertical field of view degrees */ 
               aspect_ratio:f32,
               aperature:f32) -> Self {

        let theta = vfov * (3.1415926535897932385 / 180.0);
        let h = (theta / 2.0).tan();

        let vp_height = 2.0 * h;
        let vp_width = aspect_ratio * vp_height;

        let view = target - origin;
        let focus_distance = cgmath::dot(view, view).abs().sqrt();

        let direction = view.normalize();
        let h_unit = direction.cross(up).normalize();
        let v_unit = h_unit.cross(direction);

        Camera {
            origin,
            h_unit,
            v_unit,
            vp_center:     focus_distance * direction,
            vp_horizontal: focus_distance * vp_width * h_unit,
            vp_vertical:   focus_distance * vp_height * v_unit,
            lens_radius:   aperature / 2.0,
        }
    }

    pub fn get_ray(&self, u:f32, v:f32) -> Ray {
        let direction = self.vp_center 
            + (u-0.5) * self.vp_horizontal 
            + (v-0.5) * self.vp_vertical;
        let rdisk = self.lens_radius * random_in_unit_disk();
        let offset = self.h_unit*rdisk.x + self.v_unit*rdisk.y; 
        Ray::new(self.origin + offset, direction - offset)
    }
}
