use std::fs;

use eframe::egui;

use sgs::board::Board;
use sgs::panel::Panel;
use sgs::speech::SpeechEngine;

struct App {
    speech_engine: SpeechEngine,
    panel: Panel,
    home: Board,
    board: Board,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let speech_engine = SpeechEngine::new();
        let panel = Panel::new();

        let path = "board.json";
        let home : Board = fs::read_to_string(path).map(|contents|
            Board::load_str(&contents).unwrap()
        ).unwrap();
        let board = home.clone();

        Self { speech_engine, panel, home, board }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for entry in &self.panel.entries {
                    ui.button(entry.label.clone());
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("board").show(ui, |ui| {
                let built = self.board.build().unwrap();
                for row in 0..built.layout.rows {
                    for col in 0..built.layout.cols {
                        if let Some(button) = built.get_button(col, row) {
                            if ui.button(button.label.clone()).clicked() {
                                self.panel.apply_button(button, &mut self.speech_engine);
                            }
                        } else {
                            // No button for (row, col).
                        }
                    }
                    ui.end_row();
                }

                //Ok(self.board.clone());
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("AACApp", native_options, Box::new(|cc| Box::new(App::new(cc))));
}
