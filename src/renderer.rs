
use crate::image::Image;
use crate::image::Color;
use crate::camera::Camera;
use crate::scene::Scene;

type Vector3 = cgmath::Vector3<f32>;
use cgmath::InnerSpace;

pub struct Renderer {
    nsamples: usize,
    max_depth: usize,
    rands: Vec<f32>,
}

pub struct RenderTarget {
    pub full_width: usize,
    pub full_height: usize,
    pub buffer: Image,
}

impl Renderer {
    pub fn new(nsamples:usize, max_depth:usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rands: Vec<f32> = Vec::with_capacity(nsamples+1);
        rands.push(0.0);
        rands.push(0.0);
        for _ in 2..rands.capacity()+1 {
            rands.push(rng.gen_range(-0.5..0.5));
        }
        Renderer {
            nsamples,
            max_depth,
            rands,
        }
    }
    pub fn render(&self, camera:&Camera, scene:&Scene, target:&RenderTarget) {
        for (x,y) in &target.buffer {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for i in 0..self.nsamples {
                let u = (x as f32 + self.rands[i+0]) / (target.full_width-1) as f32;
                let v = (y as f32 + self.rands[i+1]) / (target.full_height-1) as f32;
                let ray = camera.get_ray(u,v);
                color = color + scene.cast(&ray, self.max_depth);
            }
            // scale and gamma correction (gamma=2)
            let scale = 1.0 / self.nsamples as f32;
            color = Color {
                red:   (scale*color.red).sqrt(),
                green: (scale*color.green).sqrt(),
                blue:  (scale*color.blue).sqrt(),
            };
            target.buffer.set_pixel_color(x, y, color);
        }
    }
}

use rand::Rng;

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