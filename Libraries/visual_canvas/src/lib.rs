mod display;
mod png;
mod page_builder;
mod components;

use crate::display::{ScreenSize, Display};
use crate::png::{Asset, Assets};

pub mod prelude {
    pub use crate::png::{Asset, Assets};
    pub use crate::Canvas;
    pub use crate::display::{ScreenSize, Display};
}

pub struct Canvas {
    screen_size: ScreenSize,
    pub display: Display,
    name: String,
    assets: Assets,
}

impl Canvas {
    pub fn new(screen_size: ScreenSize, name: String, assets: Option<Assets>) -> Self {
        let display = Display::new(&name, screen_size);

        let assets = assets.unwrap_or_else(|| Assets::new());

        Canvas { screen_size, display, name, assets }
    }

    pub fn render_canvas(&mut self) {
        let assets = self.assets.get_keys();

        for asset in assets.iter() {
            self.render_asset(asset);
        }
        self.display.update();
    }
}