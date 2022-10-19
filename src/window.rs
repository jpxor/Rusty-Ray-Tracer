
use minifb::{Key, Window, WindowOptions};
use crate::image::Image;
use std::sync::Arc;

pub fn open(name:&str, width:usize, height:usize, rx:std::sync::mpsc::Receiver<u32>, img:Arc<Image>) {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        name,
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~24 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(41667)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let _ = rx.recv().unwrap();
        for i in 0..buffer.len() {
            buffer[i] = img.get_pixel_u32(3*(buffer.len()-i-1));
        }
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
