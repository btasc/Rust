mod pages;

use pages::settings_layout;

use visuals_core_2::prelude::*;

fn main() {
    let mut page = Page::from_layout(ScreenSize::W720, "Yo", settings_layout());
    
    for i in 0..3600 {
        page.render();
    }
}