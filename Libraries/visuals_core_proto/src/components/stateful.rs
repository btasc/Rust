use crate::screen::asset::Asset;
use crate::Page;
use crate::components::component::{Component, ComponentBundle, ComponentLayout};

pub enum Stateful {
    Button(Asset, Asset, usize, usize, fn(&Page) -> bool),
    MultiButton, // Multiple buttons, one selected at a time
}

pub struct Button {
    click_buffer: bool,
    state: bool,
    true_asset: usize, // Index
    false_asset: usize, // Index
    x: usize, y: usize,
    check_state: fn(&Page) -> bool,
}

pub struct MultiButton {

}

impl Component for Button {
    fn get_asset(&self) -> usize {
        if self.state {
            self.true_asset
        } else {
            self.false_asset
        }
    }

    fn run_logic(&mut self, page: &Page) {
        self.check_state(page);
    }
}

impl Button {
    pub fn from(true_asset: usize, false_asset: usize, x: usize, y: usize, check_state: fn(&Page) -> bool) -> Button {
        Button { true_asset, false_asset, x, y, check_state, click_buffer: false, state: false, }
    }
    
    fn flip_state(&mut self) {
        self.state = !self.state;
    }

    pub fn check_state(&mut self, page: &Page) {
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