
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;
use threadpool::ThreadPool;

use rustytracer::utils;
use rustytracer::camera::Camera;
use rustytracer::scene::Scene;
use rustytracer::image::Image;
use rustytracer::renderer::Renderer;
use rustytracer::renderer::RenderTarget;

type Vector3 = cgmath::Vector3<f32>;
use cgmath::InnerSpace;

fn main() {
    
    let outpath = "traced.bmp";

    println!("Raytracer In a Weekend!");
    println!("output: {}", outpath);

    let aspect = 3.0 / 2.0;
    let width = 600;
    let height = (width as f32 / aspect) as usize;

    let origin = Vector3::new(13.0, 2.0, 3.0);
    let target = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperature = 0.1;

    // update target to get focus distance of 10
    let target = 10.0 * (target-origin).normalize() + origin;

    let renderer = Arc::new(Renderer::new(32, 32));
    let img = Arc::new(Image::new(width, height));
    let scene = Arc::new(RwLock::new(Scene::new()));
    let camera = Arc::new(Camera::new(origin, target, up, vfov, aspect, aperature));

    {
        let mut scene_locked = scene.write().unwrap();
        utils::test_scene_setup(&mut scene_locked);
    }

    println!("running...");
    let timer = Instant::now();

    run(&renderer, &camera, &scene, &img);
    let elapsed = timer.elapsed().as_millis();

    img.write_bmp(outpath);
    println!("done! render time: {} ms", elapsed);   
}

fn run(renderer:&Arc<Renderer>, camera:&Arc<Camera>, scene:&Arc<RwLock<Scene>>, img:&Arc<Image>) {

    let nthreads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(nthreads);
    let njobs = renderer.nsamples;

    println!("parallelism: {}", nthreads);
    println!("njobs: {}", njobs);

    let (tx, rx) = std::sync::mpsc::channel();

    for _ in 0..njobs
    {
        let tx = tx.clone();
        let scene = scene.clone();
        let camera = camera.clone();
        let renderer = renderer.clone();

        let target = RenderTarget {
            full_width: img.width(),
            full_height: img.height(),
            buffer: Image::new(img.width(), img.height()),
        };
        pool.execute(move|| {
            let scene_readonly = scene.read().unwrap();
            renderer.render(&camera, &scene_readonly, &target);
            tx.send(target.buffer).unwrap();
        });
    }

    for i in 0..njobs {
        let partial = rx.recv().unwrap();
        img.merge(i, &partial);
        println!("\rprogress: {:.2}%", 100.0 * (i+1) as f32 / njobs as f32);
    }
}
