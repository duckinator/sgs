use eframe::egui;

use sgs::system::System;
use sgs::panel::Panel;
use sgs::speech::SpeechEngine;

const COLS: f32 = 12.0;
const ROWS: f32 = 8.0;

const BUTTON_WIDTH: f32 = 80.0;
const BUTTON_HEIGHT: f32 = 80.0;
const BUTTON_SIZE: [f32; 2] = [BUTTON_WIDTH, BUTTON_HEIGHT];

const ITEM_SPACING: f32 = 5.0;
const MARGIN: f32 = 10.0;

struct App {
    speech_engine: SpeechEngine,
    panel: Panel,
    system: System,
    current_folder: usize,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let speech_engine = SpeechEngine::default();
        let panel = Panel::default();

        let system: System = System::load_file("system.json");
        let current_folder = system.default_folder();

        Self { speech_engine, panel, system, current_folder }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let inner_margin = egui::style::Margin::same(MARGIN);
        ctx.style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(ITEM_SPACING, ITEM_SPACING);
        });
        let frame = egui::containers::Frame::central_panel(&ctx.style()).inner_margin(inner_margin);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            egui::Grid::new("main-grid").show(ui, |ui| {
                // With each [] being a sub-grid:
                // | [Speak]           | [Panel] | [Delete, Clear] |
                // -------------------------------------------------
                // | [Folder selector] | [system] | [extra]        |

                // Row 1, Column 1
                egui::Grid::new("top-left").show(ui, |ui| {
                    let egui_button = egui::Button::new("Speak");
                    if ui.add_sized(BUTTON_SIZE, egui_button).clicked() {
                        if let Err(error) = self.panel.speak(&mut self.speech_engine) {
                            panic!("{:?}", error);
                        }
                    }
                });

                // Row 1, Column 2
                egui::Grid::new("top-center").show(ui, |ui| {
                    let cols = self.system.folders[self.current_folder].cols;
                    let inner_spacing = ui.ctx().style().spacing.item_spacing[0];
                    let max_width = (cols as f32) * (BUTTON_SIZE[0] + inner_spacing);

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true), |ui| {
                        ui.set_max_width(max_width);

                        for entry in &self.panel.entries {
                            // It doesn't make sense to use a button here,
                            // I just want each phrase to be distinguishable.
                            //
                            // For now, assign it to nothing.
                            // Long term, probably use a label with a background.
                            let egui_button = egui::Button::new(entry.label.clone());
                            if self.panel.entries.len() > cols {
                                let _ = ui.add(egui_button);
                            } else {
                                let _ = ui.add_sized(BUTTON_SIZE, egui_button);
                            };
                        }
                    });
                });

                // Row 1, Column 3
                egui::Grid::new("top-right").show(ui, |ui| {
                    let egui_button = egui::Button::new("Delete");
                    if ui.add_sized(BUTTON_SIZE, egui_button).clicked() {
                        self.panel.remove_last_entry();
                    }

                    let egui_button = egui::Button::new("Clear");
                    if ui.add_sized(BUTTON_SIZE, egui_button).clicked() {
                        self.panel.clear();
                    }
                });

                ui.end_row();

                // Row 2, Column 1
                egui::Grid::new("folder-selector-grid").show(ui, |ui| {
                    for (idx, folder) in self.system.folders.iter().enumerate() {
                        let egui_button = egui::Button::new(folder.name.clone()).selected(self.current_folder == idx);
                        if ui.add_sized(BUTTON_SIZE, egui_button).clicked() {
                            self.current_folder = idx;
                        }
                        ui.end_row();
                    }
                });

                // Row 2, Column 2
                egui::Grid::new("active-folder").show(ui, |ui| {
                    let folder = &self.system.folders[self.current_folder];
                    for row in 0..folder.rows {
                        for col in 0..folder.cols {
                            if let Some(button) = folder.get_button(col, row) {
                                let egui_button = egui::Button::new(button.label.clone());
                                if ui.add_sized(BUTTON_SIZE, egui_button).clicked() {
                                    if folder.immediate {
                                        self.speech_engine.speak(button.get_pronouncible_text()).expect("Failed to speak word");
                                    } else {
                                        self.panel.add_entry(button);
                                    }
                                }
                            } else {
                                // No button for (row, col).
                                //let egui_label = egui::Label::new("");
                                let egui_label = egui::Button::new("");
                                ui.add_sized(BUTTON_SIZE, egui_label);
                            }
                        }
                        ui.end_row();
                    }
                });

                // Row 3, Column 3
                egui::Grid::new("bottom-right").show(ui, |ui| {
                    // ...
                });
            });
        });
    }
}

fn main() {
    let width = (COLS * (BUTTON_SIZE[0] + ITEM_SPACING)) - ITEM_SPACING + (MARGIN * 2.0);
    let height = (ROWS * (BUTTON_SIZE[1] + ITEM_SPACING)) - ITEM_SPACING + (MARGIN * 2.0);

    println!("Dimensions: {}x{}", width, height);

    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(width, height)),
        ..Default::default()
    };
    eframe::run_native("AACApp", native_options, Box::new(|cc| Box::new(App::new(cc)))).expect("Could not start GUI.");
}
