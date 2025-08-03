use crate::asset::Asset;
use crate::display::Display;

// Components
use crate::Layout;

use crate::component_lib::{
    button::Button,
};

pub trait Component {
    fn get_asset(&self) -> &Asset;
    fn run_logic(&mut self, display: &Display);
    fn from_layout(layout: Layout) -> ComponentType;
}

pub struct ComponentBundle {
    components: Vec<ComponentType>
}

impl ComponentBundle {
    pub fn new() -> Self {
        ComponentBundle { components: Vec::new() }
    }

    fn add(&mut self, component: ComponentType) {
        self.components.push(component);
    }

    pub fn add_layout(&mut self, layout: Layout) {
        let component: ComponentType = Self::match_component_layout(layout);
        self.add(component);
    }

    fn match_component_layout(layout: Layout) -> ComponentType {
        match layout {
            Layout::Button(on_asset, off_asset, x, y) => Button::from_layout(Layout::Button(on_asset, off_asset, x, y)),
        }
    }

    pub fn get_assets(&self) -> Vec<&Asset> {
        let mut assets: Vec<&Asset> = Vec::new();

        for component in self.components.iter() {
            assets.push(component.get_asset());
        }

        assets
    }

    pub fn run_logics(&mut self, display: &Display) {
        for component in self.components.iter_mut() {
            component.run_logic(display);
        }
    }
}

pub enum ComponentType {
    // Stateful types
    Button(Button),
}

impl ComponentType {
    pub fn get_asset(&self) -> &Asset {
        match self {
            Self::Button(button) => &button.get_asset()
        }
    }

    pub fn run_logic(&mut self, display: &Display) {
        match self {
            Self::Button(button) => button.run_logic(display),
        }
    }
}