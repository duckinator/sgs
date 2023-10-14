use std::fs;

use eframe::egui;

use sgs::board::Board;
use sgs::panel::Panel;
use sgs::speech::SpeechEngine;

struct App {
    speech_engine: SpeechEngine,
    panel: Panel,
    board: Board,
    current_layout: usize,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let speech_engine = SpeechEngine::new();
        let panel = Panel::new();

        let path = "board.json";
        let board: Board = fs::read_to_string(path).map(|contents|
            Board::load_str(&contents).unwrap()
        ).unwrap();
        let current_layout = board.default_layout();

        Self { speech_engine, panel, board, current_layout }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("panel").show(ctx, |ui| {
            egui::Grid::new("panel-grid").show(ui, |ui| {
                if ui.button("Speak").clicked() {
                    if let Err(error) = self.panel.speak(&mut self.speech_engine) {
                        panic!("{:?}", error);
                    }
                }

                ui.horizontal(|ui| {
                    for entry in &self.panel.entries {
                        // It doesn't make sense to use a button here,
                        // I just want each phrase to be distinguishable.
                        //
                        // For now, assign it to nothing.
                        // Long term, probably use a label with a background.
                        let _ = ui.button(entry.label.clone());
                    }
                });

                if ui.button("Delete").clicked() {
                    self.panel.remove_last_entry();
                }

                if ui.button("Clear").clicked() {
                    self.panel.clear();
                }
            });
        });

        egui::SidePanel::left("layout-selector").show(ctx, |ui| {
            egui::Grid::new("layout-selector-grid").show(ui, |ui| {
                for (idx, layout) in self.board.layouts.iter().enumerate() {
                    ui.selectable_value(&mut self.current_layout, idx, layout.name.clone());
                    ui.end_row();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("board").show(ui, |ui| {
                let layout = &self.board.layouts[self.current_layout];
                for row in 0..layout.rows {
                    for col in 0..layout.cols {
                        if let Some(button) = layout.get_button(col, row) {
                            if ui.button(button.label.clone()).clicked() {
                                if layout.immediate {
                                    self.speech_engine.speak(button.get_pronouncible_text()).expect("Failed to speak word");
                                } else {
                                    self.panel.add_entry(button);
                                }
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
    eframe::run_native("AACApp", native_options, Box::new(|cc| Box::new(App::new(cc)))).expect("Could not start GUI.");
}
