use std::path::Path;
use image::RgbaImage;

pub struct Asset {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Asset { // Duplicate code is in visual_core_proto
    pub fn load(path: &str) -> Asset {
        let img = image::open(&Path::new(path))
            .expect("Failed to open image")
            .to_rgba8();

        let buffer = Self::convert_to_buffer(&img);
        let (width , height) = img.dimensions();
        let (width, height) = (width as usize, height as usize);

        Asset { buffer, width, height }
    }

    fn convert_to_buffer(img: &RgbaImage) -> Vec<u32> {
        img.chunks_exact(4)
            .map(|chunk| {
                let r = chunk[0] as u32;
                let g = chunk[1] as u32;
                let b = chunk[2] as u32;
                let a = chunk[3] as u32;
                (a << 24) | (r << 16) | (g << 8) | b
            })
            .collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}