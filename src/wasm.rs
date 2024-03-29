#![cfg(target_arch = "wasm32")]

use crate::app::App;

use eframe::egui;
use log;
use wasm_bindgen::{self, prelude::*};

/// Your handle to the web app from JavaScript.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    /// Installs a panic hook, then returns.
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Redirect [`log`] message to `console.log` and friends:
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    /// Call this once from JavaScript to start your app.
    #[wasm_bindgen]
    pub async fn start(&self, canvas_id: &str, max_width: f32, max_height: f32) -> Result<(), wasm_bindgen::JsValue> {
        let web_options = eframe::WebOptions {
            max_size_points: egui::vec2(max_width, max_height),
            ..Default::default()
        };

        self.runner
            .start(
                canvas_id,
                web_options,
                Box::new(|cc| Box::new(App::new(cc))),
            )
            .await
    }

    // The following are optional:

    /// Shut down eframe and clean up resources.
    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }

    /*
    /// Example on how to call into your app from JavaScript.
    #[wasm_bindgen]
    pub fn example(&self) {
        if let Some(app) = self.runner.app_mut::<App>() {
            app.example();
        }
    }
    */

    /// The JavaScript can check whether or not your app has crashed:
    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }
}
