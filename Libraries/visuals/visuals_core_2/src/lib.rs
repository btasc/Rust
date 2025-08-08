mod display;
mod asset;
mod component_bundle;
mod component_lib;

use std::time::Duration;
use std::thread;

use crate::asset::Asset;
use crate::component_bundle::{Component, ComponentBundle};
use crate::display::{Display, ScreenSize};

pub mod prelude {
    pub use crate::{Page, Layout};
    pub use crate::asset::Asset;
    pub use crate::display::ScreenSize;
}

pub struct Page {
    display: Display,
    components: ComponentBundle,
}

impl Page {
    pub fn new(title: &str, screen_size: ScreenSize) -> Page {
        Page { display: Display::new(title, screen_size), components: ComponentBundle::new() }
    }

    pub fn from_layout(screen_size: ScreenSize, name: &str, layouts: Vec<Layout>) -> Page {
        let mut page = Self::new(name, screen_size);

        for layout in layouts {
            page.components.add_layout(layout);
        }

        page
    }

    pub fn render(&mut self) {
        self.components.run_logics(&self.display);
        let components = self.components.get_components();

        for component in components {
            let asset = component.get_asset();
            
            let x = component.x();
            let y = component.y();

            self.display.render_asset(asset, x, y);
        }

        self.display.update();

        thread::sleep(Duration::from_millis(16));
    }
}

pub enum Layout {
    Button(Asset, Asset, usize, usize), // On asset, Off asset, x, y
    Image(Asset, usize, usize), // Image, x, y
}