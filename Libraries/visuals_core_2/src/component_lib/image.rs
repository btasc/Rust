use crate::display::Display;
use crate::component_bundle::{Component, ComponentType};
use crate::asset::Asset;
use crate::Layout;

pub struct Image {
    asset: Asset,
    x: usize, y: usize,
    width: usize, height: usize,
}

impl Component for Image {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn run_logic(&mut self, display: &Display) {
        ();
    }
    fn x(&self) -> usize { self.x }
    fn y(&self) -> usize { self.y }
}

impl Image {
    pub fn from_layout(layout: Layout) -> ComponentType {
        match layout {
            Layout::Image(asset, x, y) => {
                let width = asset.width();
                let height = asset.height();

                ComponentType::Image(Self { asset, x, y, width, height })
            },
            _ => unreachable!()
        }
    }
}