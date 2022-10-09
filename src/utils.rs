
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use cgmath::Vector3;

use crate::scene::Scene;
use crate::image::Color;
use crate::hittables::Sphere;
use crate::materials::Metal;
use crate::materials::Lambertian;
use crate::materials::Dialectric;

pub fn test_scene_setup(scene: &mut Scene) {
    let mut rng = SmallRng::seed_from_u64(0);

     // ground
     let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
     scene.push(
         Sphere::new( Vector3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground )
     );

     // little balls randomly strewn about
     for a in -11..11 {
         for b in -11..11 {
             let rand_mat = rng.gen_range(0.0..1.0);
             let rand_a = rng.gen_range(0.0..1.0);
             let rand_b = rng.gen_range(0.0..1.0);

             let center = Vector3::new(a as f32 +0.9*rand_a, 0.2, b as f32+0.9*rand_b);
             let offset = center-Vector3::new(4.0, 0.2, 0.0);

             if cgmath::dot(offset, offset) > (0.9*0.9) {
                 if rand_mat < 0.8 {
                     // diffuse
                     let albedo = Color::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
                     let mat = Lambertian::new(albedo);
                     scene.push(
                         Sphere::new(center, 0.2, mat)
                     );
                 } else if rand_mat < 0.95 {
                     // metal
                     let albedo = Color::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
                     let roughness = rng.gen_range(0.0..1.0);
                     let mat = Metal::new(albedo, roughness);
                     scene.push(
                         Sphere::new(center, 0.2, mat)
                     );
                 } else {
                     // glass
                     let mat = Dialectric::new(1.5);
                     scene.push(
                         Sphere::new(center, 0.2, mat)
                     );
                 }
             }
         }
     }

     // the big balls
     scene.push(Sphere::new(
         Vector3::new(0.0, 1.0, 0.0), 1.0, Dialectric::new(1.5)
     ));
     scene.push(Sphere::new(
         Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Color::new(0.4, 0.2, 0.1))
     ));
     scene.push(Sphere::new(
         Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)
     ));
}