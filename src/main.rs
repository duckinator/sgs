#[cfg(not(target_arch = "wasm32"))]
use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use sgs::app::{
    COLS,
    ROWS,
    ITEM_SPACING,
    MARGIN,
    MIN_WIDTH,
    MIN_HEIGHT,
    BUTTON_WIDTH,
    BUTTON_HEIGHT,
    App,
};

// See https://github.com/rust-lang/rust/issues/103336
#[cfg(target_arch = "wasm32")]
fn main() { }

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let width = (COLS * (BUTTON_WIDTH + ITEM_SPACING)) - ITEM_SPACING + (MARGIN * 2.0);
    let height = (ROWS * (BUTTON_HEIGHT + ITEM_SPACING)) - ITEM_SPACING + (MARGIN * 2.0);

    println!("Button size: {}x{}", BUTTON_WIDTH, BUTTON_HEIGHT);
    println!("Window size: {}x{}", MIN_WIDTH, MIN_HEIGHT);
    println!("Needed size: {}x{}", width, height);

    assert!(width <= MIN_WIDTH);
    assert!(height <= MIN_HEIGHT);

    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(MIN_WIDTH, MIN_HEIGHT)),
        ..Default::default()
    };

    eframe::run_native("AACApp", native_options, Box::new(|cc| Box::new(App::new(cc)))).expect("Could not start GUI.");
}
