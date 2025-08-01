use crate::components::dynamic::Dynamic;
use crate::components::statefull::Statefull;
use crate::canvas::asset::{Asset, AssetBundle};

pub enum Component {
    Statefull(Statefull),
    Dynamic(Dynamic),
    Static(Asset, u32, u32)
}

pub trait GetAsset {
    fn get_asset(&self) -> usize; // Index of asset in bundle
}

struct Static {
    asset: Asset,
    x: u32, y: u32
}