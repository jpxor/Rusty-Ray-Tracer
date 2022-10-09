
use crate::ray::Ray;
use crate::image::Image;
use crate::image::Color;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::hittables::HitRecord;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use cgmath::InnerSpace;

type Vector3 = cgmath::Vector3<f32>;

pub struct Renderer {
    nsamples: usize,
    max_depth: usize,
    tmin:f32,
    tmax:f32,
}

pub struct RenderTarget {
    pub full_width: usize,
    pub full_height: usize,
    pub buffer: Image,
}

impl Renderer {

    pub fn new(nsamples:usize, max_depth:usize) -> Self {
        Renderer {
            nsamples,
            max_depth,
            tmin: 0.001,
            tmax: 1000.0,
        }
    }

    pub fn render(&self, camera:&Camera, scene:&Scene, target:&RenderTarget) {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut rands_u: Vec<f32> = Vec::with_capacity(self.nsamples);
        let mut rands_v: Vec<f32> = Vec::with_capacity(self.nsamples);

        let inv_w = 1.0 / (target.full_width-1) as f32;
        let inv_h = 1.0 / (target.full_height-1) as f32;
        let scale = 1.0 / self.nsamples as f32;

        rands_u.push(0.0);
        rands_v.push(0.0);
        for _ in 1..self.nsamples {
            rands_u.push(rng.gen_range(-0.5..0.5) * inv_w);
            rands_v.push(rng.gen_range(-0.5..0.5) * inv_h);
        }

        for y in target.buffer.y_range_iter() {
            let v = y as f32 * inv_h;

            for x in target.buffer.x_range_iter() {
                let u = x as f32 * inv_w;
                let mut color = Color::black();

                for i in 0..self.nsamples {
                    let ray = camera.get_ray(u+rands_u[i], v+rands_v[i]);
                    color = color + self.cast(scene, &ray, self.max_depth);
                }

                // scale and gamma correction (gamma=2)
                color = Color {
                    red:   (scale*color.red).sqrt(),
                    green: (scale*color.green).sqrt(),
                    blue:  (scale*color.blue).sqrt(),
                };
                target.buffer.set_pixel_color(x, y, color);
            }
        }
    }

    pub fn cast(&self, scene:&Scene, ray:&Ray, depth:usize) -> Color {
        if depth == 0 {
            return self.on_miss(ray);
        }
        match scene.hit(ray, self.tmin, self.tmax) {
            None => self.on_miss(ray),
            Some(hit) => self.on_hit(scene, ray, depth, hit),
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
    
    pub fn on_hit(&self, scene:&Scene, ray:&Ray, depth:usize, hit:HitRecord) -> Color {
        match hit.material.scatter(ray, &hit) {
            None => Color::black(),
            Some(scatter) => {
                scatter.attenuation * self.cast(scene, &scatter.ray, depth-1)
            }
        }
    }

}

pub fn random_in_unit_sphere_vector3() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut randv:Vector3;
    loop {
        randv = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if cgmath::dot(randv, randv) >= 1.0 {
            continue
        }
        return randv;
    }
}

pub fn random_unit_vector3() -> Vector3 {
    random_in_unit_sphere_vector3().normalize()
}