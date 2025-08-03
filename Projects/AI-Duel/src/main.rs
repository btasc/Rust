use visuals_core_2::prelude::*;

fn main() {
    let mut page = Page::from_layout(ScreenSize::W720, "Yo", get_layout());
    
    for i in 0..3600 {
        page.render();
    }
}

fn get_layout() -> Vec<Layout> {
    let on_asset = Asset::load("./assets/components/ButtonOn1.png");
    let off_asset = Asset::load("./assets/components/ButtonOff1.png");
    
    let button_layout = Layout::Button(on_asset, off_asset, 20, 20);
    
    vec![button_layout]
}