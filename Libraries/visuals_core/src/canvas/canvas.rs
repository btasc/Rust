use crate::canvas::display::Display;
use crate::canvas::asset::Asset;

pub struct Canvas {
    display: Display
}

impl Canvas {
    pub fn new(name: &String, screen_size: ScreenSize) -> Canvas  {
        let display = Display::new(name, screen_size);

        Canvas { display }
    }

    pub fn render_asset(&mut self, asset: &Asset, x: usize, y: usize) {

        assert!(ScreenSize::W180.width() >= x + asset.width() as usize);
        assert!(ScreenSize::W180.height() >= y + asset.height() as usize);

        let buffer = asset.get_buffer();

        for i in 0..buffer.len() {
            let asset_x = i % asset.width() as usize;
            let asset_y = i / asset.width() as usize;

            let hex = buffer[i];
            self.display.set_pixel(x + asset_x, y + asset_y, hex);
        }
    }
}






// !-- Screen Size --! //
pub enum ScreenSize {
    W180, W360, W540, W720, W900, W1080, W1440, W2160, W4320
}

impl ScreenSize {
    pub fn width(&self) -> usize {
        match self {
            ScreenSize::W180  => 320,  ScreenSize::W360  => 640,
            ScreenSize::W540  => 960,  ScreenSize::W720  => 1280,
            ScreenSize::W900  => 1600, ScreenSize::W1080 => 1920,
            ScreenSize::W1440 => 2560, ScreenSize::W2160 => 3840,
            ScreenSize::W4320 => 7680,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            ScreenSize::W180  => 180,  ScreenSize::W360  => 360,
            ScreenSize::W540  => 540,  ScreenSize::W720  => 720,
            ScreenSize::W900  => 900,  ScreenSize::W1080 => 1080,
            ScreenSize::W1440 => 1440, ScreenSize::W2160 => 2160,
            ScreenSize::W4320 => 4320,
        }
    }

    pub fn get_buff_len(&self) -> usize {
        self.width() * self.height()
    }

    pub fn get_scale(&self) -> usize {
        match self {
            ScreenSize::W180  => 1,  ScreenSize::W360  => 2,
            ScreenSize::W540  => 3,  ScreenSize::W720  => 4,
            ScreenSize::W900  => 5,  ScreenSize::W1080 => 6,
            ScreenSize::W1440 => 8,  ScreenSize::W2160 => 12,
            ScreenSize::W4320 => 24,
        }
    }
}