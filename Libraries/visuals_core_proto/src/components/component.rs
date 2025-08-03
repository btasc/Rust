use crate::Page;
use crate::screen::asset::{Asset, AssetBundle};

use crate::components::stateful::{Stateful, Button, MultiButton};
use crate::components::sprite::Sprite;

// !-- Bundles --! //

pub struct ComponentBundle {
    pub dynamic: Vec<Sprite>,
    pub stateful: StatefulBundle,
    pub image: Vec<Image>,
}

impl ComponentBundle {
    pub fn new() -> Self {
        ComponentBundle {
            dynamic: Vec::new(),
            stateful: StatefulBundle::new(),
            image: Vec::new()
        }
    }

    pub fn from(components: Vec<ComponentLayout>) -> (ComponentBundle, AssetBundle)  {
        let mut asset_list: Vec<Asset> = Vec::new();
        let mut component_bundle = ComponentBundle::new();

        let mut asset_bundle_index: usize = 0;

        for component in components {
            match component {
                ComponentLayout::Image(_, _, _) => (),
                ComponentLayout::Stateful(stateful) => {
                    match stateful {
                        Stateful::MultiButton => {
                            ()
                        },
                        Stateful::Button(true_asset, false_asset, x, y, check_state) => {
                            asset_list.push(true_asset);
                            let true_asset_index = asset_bundle_index;
                            asset_bundle_index += 1;

                            asset_list.push(false_asset);
                            let false_asset_index = asset_bundle_index;
                            asset_bundle_index += 1;

                            let button = Button::from(true_asset_index, false_asset_index, x, y, check_state);
                            component_bundle.stateful.buttons.push(button);
                        }
                    }
                }
                ComponentLayout::Sprite => (),
            }
        }

        (component_bundle, AssetBundle::bundle(asset_list))
    }
    
    pub fn run_logic(&mut self, page: &Page) {
        for i in 0..self.dynamic.len() {
            ();
        }
        
        for i in 0..self.image.len() {
            ();
        }
        
        self.stateful.run_logic(page);
    }
}

pub struct StatefulBundle {
    pub buttons: Vec<Button>,
    pub multi_buttons: Vec<MultiButton>,
}

impl StatefulBundle {
    pub fn new() -> Self {
        StatefulBundle { buttons: Vec::new(), multi_buttons: Vec::new() }
    }
    
    pub fn run_logic(&mut self, page: &Page) {
        for i in 0..self.multi_buttons.len() {
            ();
        }
        
        for button in self.buttons.iter_mut() {
            button.check_state(page);
        }
    }
}

pub trait Component {
    fn get_asset(&self) -> usize; // Index of asset in bundle
    fn run_logic(&mut self, page: &Page);
}

// !-- Component Layout --! //

pub enum ComponentLayout {
    Stateful(Stateful),
    Sprite,
    Image(Asset, u32, u32)
}

impl ComponentLayout {
    pub fn button(true_asset: Asset, false_asset: Asset, x: usize, y: usize, check_state: fn(&Page) -> bool) -> ComponentLayout {
        ComponentLayout::Stateful(Stateful::Button(true_asset, false_asset, x, y, check_state))
    }
}

pub struct Image {
    asset: Asset,
    x: u32, y: u32
}