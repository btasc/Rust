use minifb::{Window, WindowOptions};

use crate::canvas::canvas::ScreenSize;

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
        assert!(x <= ScreenSize::W180.width());
        assert!(y < ScreenSize::W180.height());

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

    pub fn get_mouse_pos(&self) ->  Option<(f32, f32)> {
        self.window.get_mouse_pos(minifb::MouseMode::Discard)
    }

    pub fn get_mouse_state(&self) -> bool {
        self.window.get_mouse_down(minifb::MouseButton::Left)
    }
}