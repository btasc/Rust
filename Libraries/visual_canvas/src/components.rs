use crate::Canvas;
use crate::png::{Asset};

enum Component {
    Button(Button),
    Static(Static),
    Dynamic(Dynamic),
}

trait ComponentTrait {
    fn get_asset_name(&self) -> &String;
}

// !-- Button Component --! //
// Statefull, can change (Such as a checkmark or something)

struct Button {
    state: bool,
    true_asset: Asset,
    false_asset: Asset,
    check_state: fn() -> bool,
    switch_buffer: bool,
}

impl ComponentTrait for Button {
    fn get_asset_name(&self) -> &String {
        if self.state {
            &self.true_asset.name
        } else {
            &self.false_asset.name
        }
    }
}

impl Button {
    pub fn new(state: bool, true_asset: Asset, false_asset: Asset, check_state: fn() -> bool) -> Button {
        Button { state, true_asset, false_asset, check_state, switch_buffer: false }
    }

    pub fn check_then_flip(&mut self) {
        if (self.check_state)() {  // Has to be wrapped in parentheses because it's a field not a method
            if !self.switch_buffer {
                self.flip_state();

                self.switch_buffer = true;
            }
        } else {
            self.switch_buffer = false;
        }
    }

    pub fn flip_state(&mut self) {
        self.state = !self.state;
    }
}

// !-- Static Component --! //
// Static, image or background piece

struct Static {

}

// !-- Dynamic Component --! //
// Dynamic, asset that's moving in some way

struct Dynamic {

}