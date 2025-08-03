mod components;
mod screen;
use std::path::{Component, Components};
use crate::components::component::{ComponentBundle, ComponentLayout};
use crate::screen::{
    asset::AssetBundle,
    display::{
        ScreenSize,
        Display
    }
};

use std::time::Duration;
use std::thread;

pub mod prelude {
    pub use crate::Page;
    pub use crate::screen::display::ScreenSize;
    pub use crate::components::component::ComponentLayout;
    pub use crate::screen::asset::Asset;
}

pub struct Page {
    display: Display,
    title: String,
    asset_bundle: AssetBundle,
    component_bundle: ComponentBundle,
}

impl Page {
    pub fn from(title: String, screen_size: ScreenSize, components: Vec<ComponentLayout>) -> Page {
        let display = Display::new(&title, screen_size);
        let (component_bundle, asset_bundle) = ComponentBundle::from(components);

        Page { display, title, asset_bundle, component_bundle }
    }

    pub fn render_loop(&mut self) {
        self.display.update();

        thread::sleep(Duration::from_millis(10));
    }

    fn logic_loop(&mut self) {
        self.component_bundle.run_logic(&self.display);
    }

    fn buffer_loop(&mut self) {

    }
}