use std::path::{Path, PathBuf};

use visuals_core_2::prelude::*;

const ASSET_ROOT: &'static str = "./assets/";

// !-- Settings Page --! //
pub fn settings_layout() -> Vec<Layout> {
    let root_path = Path::new(ASSET_ROOT);
    let mut layout: Vec<Layout> = Vec::new();

    let background_asset = Asset::load(root_path.join("backgrounds/settings.png"));
    let background = Layout::Image(background_asset, 0, 0);

    layout.push(background);

    let gpu_button_on_asset = Asset::load(root_path.join("components/CheckBoxOn1.png"));
    let gpu_button_off_asset = Asset::load(root_path.join("components/CheckBoxOff1.png"));

    let gpu_button = Layout::Button(gpu_button_on_asset, gpu_button_off_asset, 91, 58);

    layout.push(gpu_button);

    layout
}