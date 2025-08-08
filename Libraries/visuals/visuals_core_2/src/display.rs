use minifb::{Window, WindowOptions};
use crate::asset::Asset;

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    screen_size: ScreenSize,
}

impl Display {
    pub fn new(title: &str, screen_size: ScreenSize) -> Self {
        let window = Window::new(
            title,
            screen_size.width(),
            screen_size.height(),
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
            .expect("Unable to create window");

        let buffer = vec![0; screen_size.get_buff_len()];

        Self { window, buffer, screen_size }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.screen_size.width(), self.screen_size.height())
            .expect("Failed to update window");
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        assert!(x <= ScreenSize::W180.width(), "Expected x to be smaller than ScreenSize w180.width, x was equal to: {}", x);
        assert!(y <= ScreenSize::W180.height(), "Expected y to be smaller than ScreenSize w180.height, y was equal to: {}", y);

        let scale = self.screen_size.get_scale();

        for h in 0..scale {
            for k in 0..scale {
                self.buffer[(y*scale + k) * self.screen_size.width() + (x*scale + h)] = color;
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape)
    }

    pub fn get_mouse_pos(&self) ->  Option<(f32, f32)> { self.window.get_mouse_pos(minifb::MouseMode::Discard) }

    pub fn get_mouse_state(&self) -> bool {
        self.window.get_mouse_down(minifb::MouseButton::Left)
    }

    pub fn render_asset(&mut self, asset: &Asset, x: usize, y: usize) {
        let buffer = asset.get_buffer();

        for i in 0..buffer.len() {
            let asset_x = i % asset.width() as usize;
            let asset_y = i / asset.width() as usize;

            let hex = buffer[i];
            self.set_pixel(x + asset_x, y + asset_y, hex);
        }
    }

    pub fn contains_point(&self, x: usize, y: usize, width: usize, height: usize) -> bool {
        if let Some((mx, my)) = self.get_mouse_pos() {
            let scale = self.screen_size.get_scale();

            let (x, y, width, height) = (x * scale, y * scale, width * scale, height * scale);

            if mx < 0f32 || my < 0f32 {
                return false;
            }

            let (mx, my) = (mx as usize, my as usize);
            mx >= x && mx < x + width && my >= y && my < y + height
        } else {
            false
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