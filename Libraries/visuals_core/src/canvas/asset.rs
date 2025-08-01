use image::RgbaImage;
use std::path::Path;
use std::collections::HashMap;

pub struct Asset {
    buffer: Vec<u32>,
    width: u32,
    height: u32,
}

pub struct AssetBundle {
    bundle: HashMap<usize, Asset>
}

impl Asset {
    pub fn load(path: &str) -> Asset {
        let img = image::open(&Path::new(path))
            .expect("Failed to open image")
            .to_rgba8();

        let buffer = Self::convert_to_buffer(&img);
        let (width, height) = img.dimensions();

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
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

impl AssetBundle {
    fn new() -> AssetBundle {
        AssetBundle { bundle: HashMap::new() }
    }

    pub fn bundle(assets: Vec<Asset>) -> AssetBundle {
        let mut bundle = AssetBundle::new();

        for (i, asset) in assets.into_iter().enumerate() {
            bundle.bundle.insert(i, asset);
        }

        bundle
    }
    
    pub fn get_asset(&self, id: usize) -> Option<&Asset> {
        self.bundle.get(&id)
    }
}