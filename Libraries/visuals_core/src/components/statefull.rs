use minifb::Key::P;
use crate::canvas::asset::{Asset, AssetBundle};
use crate::components::component::GetAsset;
use crate::Page;

pub enum Statefull {
    Button(Asset, Asset, u32, u32, fn(&Page) -> bool),
    MultiButton, // Multiple buttons, one selected at a time
}

struct Button {
    click_buffer: bool,
    state: bool,
    true_asset: usize, // Index
    false_asset: usize, // Index
    x: u32, y: u32,
    check_state: fn(&Page) -> bool,
}

impl GetAsset for Button {
    fn get_asset(&self) -> usize {
        if self.state {
            self.true_asset
        } else {
            self.false_asset
        }
    }
}

impl Button {
    fn flip_state(&mut self) {
        self.state = !self.state;
    }

    fn check_state(&mut self, page: &Page) {
        let is_clicked = (self.check_state)(page);

        if is_clicked {
            if !self.click_buffer {
                self.click_buffer = true;
                self.flip_state();
            }
        } else {
            self.click_buffer = false;
        }
    }
}