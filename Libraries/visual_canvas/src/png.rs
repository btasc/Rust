use std::collections::HashMap;
use std::path::Path;

use crate::Canvas;
use crate::display::{ScreenSize};

use image::RgbaImage;

pub struct Assets {
    asset_map: HashMap<String, Asset>,
}

pub struct Asset {
    path: String,
    pub name: String,
    buffer: Vec<u32>,
    width: u32,
    height: u32,
    x: usize,
    y: usize,
}

pub struct SpriteSheet {

}

impl Asset {
    pub fn load_image(path: &str, name: &str) -> Asset {
        let img = image::open(&Path::new(path))
            .expect("Failed to open image")
            .to_rgba8();

        let buffer = Self::convert_to_buffer(&img);
        let (width, height) = img.dimensions();

        Asset {
            path: path.to_string(),
            name: name.to_string(),
            buffer,
            width,
            height,
            x: 0,
            y: 0,
        }
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
}

impl Assets {
    pub fn new() -> Self {
        Assets { asset_map: HashMap::new() }
    }

    pub fn collect_assets(asset_vec: Vec<Asset>) -> Assets {
        let mut assets = Assets::new();

        for asset in asset_vec.into_iter() {
            assets.insert_asset(asset);
        }

        assets
    }

    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        self.asset_map.get(name)
    }

    pub fn get_asset_buffer(&self, name: &str) -> Option<&Vec<u32>> {
        let asset_option = self.get_asset(name);

        match asset_option {
            Some(asset) => Some(&asset.buffer),
            None => None
        }
    }

    pub fn insert_asset(&mut self, asset: Asset) {
        self.asset_map.insert(asset.name.clone(), asset);
    }
    
    pub fn get_keys(&self) -> Vec<String> {
        self.asset_map.keys().cloned().collect()
    }
}

impl Canvas {
    pub fn render_asset(&mut self, name: &str) {
        let asset = self.assets.get_asset(name).expect("Failed to get asset at render_asset");
        let (x, y) = (asset.x, asset.y);

        assert!(ScreenSize::W180.width() >= x + asset.width as usize);
        assert!(ScreenSize::W180.height() >= y + asset.height as usize);

        for i in 0..asset.buffer.len() {
            let asset_x = i % asset.width as usize;
            let asset_y = i / asset.width as usize;

            let hex = asset.buffer[i];
            self.display.set_pixel(x + asset_x, y + asset_y, hex);
        }
    }
}