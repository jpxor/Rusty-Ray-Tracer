#![feature(test)]

extern crate test;

use rustytracer::camera::Camera;
use rustytracer::image::Image;
use rustytracer::image::Coloru8;
use rustytracer::image::Region;
use rustytracer::renderer::Renderer;
use rustytracer::scene::Scene;
use rustytracer::renderer::RenderTarget;
use rustytracer::utils;

use cgmath::Vector3;

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn camera_get_ray(b: &mut Bencher) {
        let origin = Vector3::new(13.0, 2.0, 3.0);
        let target = Vector3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let vfov = 20.0;
        let aperature = 0.1;
        let aspect = 16.0 / 9.0;
        let u = 0.25;
        let v = 0.25;
        let camera = Camera::new(origin, target, up, vfov, aspect, aperature);
        b.iter(|| camera.get_ray(u,v));
    } // last result: 16 ns/iter (+/- 2)

    #[bench]
    fn image_set_pixel_color(b: &mut Bencher) {
        let image = Image::new(600, 400);
        let color = Coloru8{
            red:   128,
            green: 128,
            blue:  128,
        };
        let x = 300;
        let y = 200;
        b.iter(|| image.set_pixel_color_u8(x, y, color));
    } // last result: 14 ns/iter (+/- 1)

    #[bench]
    fn image_blit(b: &mut Bencher) {
        let image = Image::new(600, 400);
        let src = Image::new_with_region(Region {
            x: 100,
            y:100,
            width:300,
            height:200,
        });
        b.iter(|| image.blit(&src));
    } // last result: 144,715 ns/iter (+/- 9,983)

    #[bench]
    fn renderer_render(b: &mut Bencher) {
        let image = Image::new(8, 8);
        let renderer = Renderer::new(8, 8);

        let origin = Vector3::new(13.0, 2.0, 3.0);
        let target = Vector3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let vfov = 20.0;
        let aperature = 0.1;
        let aspect = image.width() as f32 / image.height() as f32;
        let camera = Camera::new(origin, target, up, vfov, aspect, aperature);

        let mut scene = Scene::new();
        utils::test_scene_setup(&mut scene);

        let target = RenderTarget {
            full_width: image.width(),
            full_height: image.height(),
            buffer: image,
        };

        // using: Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz, 2592 Mhz
        // before:  3,850,085 ns/iter (+/- 289,498)
        // after:   3,743,690 ns/iter (+/- 255,525)
        b.iter(|| renderer.render(&camera, &scene, &target));
    }

}