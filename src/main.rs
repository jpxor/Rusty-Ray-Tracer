
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;
use threadpool::ThreadPool;
use rand::Rng;

use rustytracer::camera::Camera;
use rustytracer::scene::Scene;
use rustytracer::image::Image;
use rustytracer::image::Region;
use rustytracer::image::Color;
use rustytracer::hittables::Sphere;
use rustytracer::renderer::Renderer;
use rustytracer::renderer::RenderTarget;

use rustytracer::materials::Metal;
use rustytracer::materials::Lambertian;
use rustytracer::materials::Dialectric;

type Vector3 = cgmath::Vector3<f32>;
use cgmath::InnerSpace;

fn main() {
    let mut rng = rand::thread_rng();
    let outpath = "traced.bmp";

    println!("Raytracer In a Weekend!");
    println!("output: {}", outpath);

    let aspect = 16.0 / 9.0;
    let width = 800;
    let height = (width as f32 / aspect) as usize;
    let img = Arc::new(Image::new(width, height));

    let origin = Vector3::new(13.0, 2.0, 3.0);
    let target = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperature = 0.1;

    // update target to get fosus distance of 10
    let target = 10.0 * (target-origin).normalize() + origin;

    let camera = Arc::new(Camera::new(origin, target, up, vfov, aspect, aperature));

    let scene = Arc::new(RwLock::new(Scene::new()));
    {
        // scope the locked scene for adding things into it
        let mut scene_locked = scene.write().unwrap();

        // ground
        let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        scene_locked.push(
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
                        scene_locked.push(
                            Sphere::new(center, 0.2, mat)
                        );
                    } else if rand_mat < 0.95 {
                        // metal
                        let albedo = Color::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
                        let roughness = rng.gen_range(0.0..1.0);
                        let mat = Metal::new(albedo, roughness);
                        scene_locked.push(
                            Sphere::new(center, 0.2, mat)
                        );
                    } else {
                        // glass
                        let mat = Dialectric::new(1.5);
                        scene_locked.push(
                            Sphere::new(center, 0.2, mat)
                        );
                    }
                }
            }
        }

        // the big balls
        scene_locked.push(Sphere::new(
            Vector3::new(0.0, 1.0, 0.0), 1.0, Dialectric::new(1.5)
        ));
        scene_locked.push(Sphere::new(
            Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Color::new(0.4, 0.2, 0.1))
        ));
        scene_locked.push(Sphere::new(
            Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)
        ));
       
    } // releases write lock on scene

    let renderer = Arc::new(Renderer::new(1, 2));
    println!("running...");
    let timer = Instant::now();

    run(&renderer, &camera, &scene, &img);
    let elapsed = timer.elapsed().as_millis();

    img.write_bmp(outpath);
    println!("done! render time: {} ms", elapsed);   
}

// benchmarks
// single thread:
// : 600x400 => 96627 ms
// split into chunks for 12 threads:
// : 64x64 => 26207 ms | 26045 ms | 25305 ms
// : 64x1  => 27585 ms | 27457 ms
// : 600x1 => 25689 ms | 26514 ms | 25785 ms
// : 32x32 => 25294 ms | 25510 ms | 26378 ms

fn run(renderer:&Arc<Renderer>, camera:&Arc<Camera>, scene:&Arc<RwLock<Scene>>, img:&Arc<Image>) {
    let regions = Region{
        x: 0, y: 0,
        width: img.width(),
        height: img.height(),
    }.chunks(64);

    let nthreads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(nthreads);
    let njobs = regions.len();

    println!("parallelism: {}", nthreads);
    println!("njobs: {}", njobs);

    let (tx, rx) = std::sync::mpsc::channel();

    for region in regions
    {
        let tx = tx.clone();
        let scene = scene.clone();
        let camera = camera.clone();
        let renderer = renderer.clone();

        let target = RenderTarget {
            full_width: img.width(),
            full_height: img.height(),
            buffer: Image::new_with_region(region),
        };
        pool.execute(move|| {
            let scene_readonly = scene.read().unwrap();
            renderer.render(&camera, &scene_readonly, &target);
            tx.send(target.buffer).unwrap();
        });
    }

    for i in 0..njobs {
        let partial = rx.recv().unwrap();
        img.blit(&partial);
        println!("\rprogress: {:.2}%", 100.0 * (i+1) as f32 / njobs as f32);
    }
}
