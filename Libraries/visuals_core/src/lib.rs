mod components;
mod canvas;

use crate::canvas::canvas::{ScreenSize, Canvas};
use crate::canvas::asset::AssetBundle;
use crate::components::component::Component;

pub struct Page {
    canvas: Canvas,
    title: String,
    bundle: AssetBundle,
}

impl Page {
    pub fn from(title: String, screen_size: ScreenSize, components: Vec<Component>) {
        let canvas = Canvas::new(&title, screen_size);

        for component in components.into_iter() {


        }
    }

    pub fn render(&mut self) {

    }
}