use crate::display::Display;
use crate::component_bundle::{Component, ComponentType};
use crate::asset::Asset;
use crate::Layout;

pub struct Button {
    on_asset: Asset,
    off_asset: Asset,
    state: bool, flip_buffer: bool,
    x: usize, y: usize,
    width: usize, height: usize,
}

impl Component for Button {
    fn get_asset(&self) -> &Asset {
        if self.state {
            &self.on_asset
        } else {
            &self.off_asset
        }
    }

    fn run_logic(&mut self, display: &Display) {
        let clicked = self.check_state(display);

        if clicked {
            if !self.flip_buffer {
                self.state = !self.state;
                self.flip_buffer = true;
            }
        } else {
            self.flip_buffer = false;
        }
    }

    fn from_layout(layout: Layout) -> ComponentType {
        match layout {
            Layout::Button(on_asset, off_asset, x, y) => {
                assert_eq!(on_asset.width(), off_asset.width());
                assert_eq!(on_asset.height(), off_asset.height());

                let width = on_asset.width();
                let height = on_asset.height();

                let button = Button { on_asset, off_asset, state: false, flip_buffer: false, x, y, width, height };

                ComponentType::Button(button)
            },
            _ => unreachable!()
        }
    }
}

impl Button {
    fn check_state(&self, display: &Display) -> bool  {
        let mouse_state = display.get_mouse_state();

        if mouse_state {
            display.contains_point(self.x, self.y, self.width, self.height)
        } else {
            false
        }
    }
}