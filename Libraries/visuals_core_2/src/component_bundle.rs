use crate::asset::Asset;
use crate::display::Display;

// Components
use crate::Layout;

use crate::component_lib::{
    button::Button,
    image::Image,
};

pub trait Component {
    fn get_asset(&self) -> &Asset;
    fn run_logic(&mut self, display: &Display);
    fn x(&self) -> usize;
    fn y(&self) -> usize;
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
            Layout::Image(asset, x, y) => Image::from_layout(Layout::Image(asset, x, y)),
        }
    }

    pub fn run_logics(&mut self, display: &Display) {
        for component in self.components.iter_mut() {
            component.run_logic(display);
        }
    }

    pub fn get_components(&mut self) -> &mut Vec<ComponentType> { &mut self.components }
}

pub enum ComponentType {
    // Stateful types
    Button(Button),
    
    // Static types
    Image(Image)
}

impl Component for ComponentType {
    fn get_asset(&self) -> &Asset {
        self.get_inner().get_asset()
    }

    fn run_logic(&mut self, display: &Display) {
        self.get_mut_inner().run_logic(display);
    }
    
    fn x(&self) -> usize {
        self.get_inner().x()
    }
    
    fn y(&self) -> usize {
        self.get_inner().y()
    }
}

impl ComponentType {
    fn get_mut_inner(&mut self) -> &mut dyn Component {
        match self {
            Self::Button(button) => button as &mut dyn Component,
            Self::Image(image) => image as &mut dyn Component,
        }
    }

    fn get_inner(&self) -> &dyn Component {
        match self {
            Self::Button(button) => button as &dyn Component,
            Self::Image(image) => image as &dyn Component,
        }
    }
}