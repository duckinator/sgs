#[cfg(not(target_arch = "wasm32"))]
use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use sgs::app::{
    MIN_WIDTH,
    MIN_HEIGHT,
    App,
};

#[cfg(target_arch = "wasm32")]
use sgs::WebHandle;

// See https://github.com/rust-lang/rust/issues/103336
#[cfg(target_arch = "wasm32")]
fn main() { }

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(MIN_WIDTH, MIN_HEIGHT)),
        ..Default::default()
    };

    eframe::run_native("AACApp", native_options, Box::new(|cc| Box::new(App::new(cc)))).expect("Could not start GUI.");
}
