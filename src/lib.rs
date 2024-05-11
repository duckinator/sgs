pub mod app;
pub mod cli;
pub mod system;
pub mod button;
pub mod panel;
pub mod speech;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::WebHandle;
