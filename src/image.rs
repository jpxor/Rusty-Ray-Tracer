
use std::ops::Mul;

use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::sync::{Mutex};

#[derive(Clone, Copy)]
pub struct Color {
    pub red:f32,
    pub green:f32,
    pub blue:f32,
}

#[derive(Clone, Copy)]
pub struct Region {
    pub x:usize,
    pub y:usize,
    pub width:usize,
    pub height:usize,
}

pub struct Image {
    region: Region,
    bytes: Mutex<Vec<u8>>,
}

pub struct RegionIter {
    x:usize,
    y:usize,
    xlimit:usize,
    ylimit:usize,
    region:Region,
}

impl IntoIterator for &Image {
    type Item = (usize, usize);
    type IntoIter = RegionIter;
    fn into_iter(self) -> Self::IntoIter {
        RegionIter {
            x: self.region.x,
            y: self.region.y,
            xlimit: self.region.x + self.region.width,
            ylimit: self.region.y + self.region.height,
            region: self.region,
        }
    }
}

impl Iterator for RegionIter {
    type Item = (usize,usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.xlimit {
            self.x = self.region.x;
            self.y += 1;
            if self.y == self.ylimit {
                return None
            }
        }
        let ret = (self.x, self.y);
        self.x += 1;
        return Some(ret);
    }
}

impl Color {
    pub fn lerp(t:f32, a:Color, b:Color) -> Color {
        (1.0-t)*a + t*b
    }
    pub fn new(red:f32, green:f32, blue:f32) -> Color {
        Color{ red, green, blue }
    }
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color{
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color { 
            red: rhs.red*self,
            green: rhs.green*self,
            blue: rhs.blue*self,
         }
    }
}
impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}


impl Region {
    pub fn chunks(&self, size:usize) -> Vec<Region> {
        let mut chunks = Vec::new();
        let mut region = Region {
            x: 0,
            y: 0,
            width: size,
            height: size,
        };
        let mut remaining_height = self.height;
        while remaining_height > 0
        {
            region.height = std::cmp::min(size, remaining_height);
            let mut remaining_width = self.width;
            while remaining_width > 0
            {
                region.width = std::cmp::min(size, remaining_width);
                chunks.push(region);

                remaining_width -= region.width;
                region.x += region.width;
            }
            remaining_height -= region.height;
            region.y += region.height;
            region.x = 0;
        }
        return chunks;
    }
}

impl<'a> Image {

    pub fn new(width:usize, height:usize) -> Image {
        Image::new_with_region( Region {
            x: 0, y:0, width, height
        })
    }
    
    pub fn new_with_region(region:Region) -> Image {
        let bytes = Mutex::new(vec![0; 3 * region.width * region.height]);
        Image { region, bytes }
    }

    pub fn width(&self) -> usize {
        self.region.width
    }

    pub fn height(&self) -> usize {
        self.region.height
    }

    pub fn blit(&self, src:&Image) {
        let x = src.region.x;
        let y = src.region.y;

        let mut dst_bytes = self.bytes.lock().unwrap();
        let dst_stride = 3*self.width();
        let mut dst_offset = 3*x + y*dst_stride;

        let src_bytes = src.bytes.lock().unwrap();
        let src_stride = 3*src.width();
        let mut src_offset = 0;

        for j in 0..src.height() {
            if y+j >= self.height() {
                break;
            }
            for i in 0..src_stride {
                if x+i >= dst_stride {
                    continue;
                }
                dst_bytes[dst_offset+i] = src_bytes[src_offset+i];
            }
            src_offset += src_stride;
            dst_offset += dst_stride;
        }
    }

    pub fn set_pixel_color(&self, x:usize, y:usize, color:Color) {
        let normalize = |f:f32| -> u8 {
            let n = (255.0 * f) as u8;
            u8::clamp(n, 0, 255)
        };

        let width = self.region.width;
        let height = self.region.height;
        let minx = self.region.x;
        let miny = self.region.y;

        if x < minx || y < miny {
            return;
        }

        let x = x - minx;
        let y = y - miny;

        if x >= width || y >= height {
            return;
        }

        let stride = 3*width;
        let y_offset = y*stride;
        let i = 3*x + y_offset;

        let mut bytes = self.bytes.lock().unwrap();

        // (B,G,R)
        bytes[i+0] = normalize(color.blue);
        bytes[i+1] = normalize(color.green);
        bytes[i+2] = normalize(color.red);
    }

    pub fn write_bmp(&self, path: &str) {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let w = self.region.width;
        let h = self.region.height;
        let filesize:u32 = 52 + (3*w*h) as u32;

        let mut file_header: [u8; 14] = ['B' as u8,'M' as u8, 0,0,0,0, 0,0, 0,0, 54,0,0,0];
        let mut info_header: [u8; 40] = [40,0,0,0, 0,0,0,0, 0,0,0,0, 1,0, 24,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
        let stride_pad: [u8; 3] = [0, 0, 0];

        file_header[ 2] = (0xFFFF & filesize>> 0) as u8;
        file_header[ 3] = (0xFFFF & filesize>> 8) as u8;
        file_header[ 4] = (0xFFFF & filesize>>16) as u8;
        file_header[ 5] = (0xFFFF & filesize>>24) as u8;

        info_header[ 4] = (0xFFFF & w >> 0) as u8;
        info_header[ 5] = (0xFFFF & w >> 8) as u8;
        info_header[ 6] = (0xFFFF & w >>16) as u8;
        info_header[ 7] = (0xFFFF & w >>24) as u8;
        info_header[ 8] = (0xFFFF & h >> 0) as u8;
        info_header[ 9] = (0xFFFF & h >> 8) as u8;
        info_header[10] = (0xFFFF & h >>16) as u8;
        info_header[11] = (0xFFFF & h >>24) as u8;

        file.write_all(&file_header).unwrap();
        file.write_all(&info_header).unwrap();

        let stride = 3*w;
        let padding = (4 - (stride) % 4) % 4;

        let bytes = self.bytes.lock().unwrap();

        for y in 0..h {
            let offset = y*stride;
            file.write_all(&bytes[offset..offset+stride]).unwrap();
            file.write_all(&stride_pad[0..padding]).unwrap();
        }
    }
}
