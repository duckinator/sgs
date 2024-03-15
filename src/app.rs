use log::info; // also available: trace, warn
use eframe::egui;
use eframe::egui::Pos2;

use crate::system::System;
use crate::panel::Panel;
use crate::speech::SpeechEngine;

pub const MIN_WIDTH: f32 = 1280.0;
pub const MIN_HEIGHT: f32 = 720.0;

#[derive(Debug)]
struct Dimensions {
    item_spacing: f32,
    margin: f32,
    width: f32,
    height: f32,
    button_size: [f32; 2],
    rows: usize,
    cols: usize,
}

impl Dimensions {
    pub fn new(screen_size: Pos2, rows: usize, cols: usize) -> Self {
        let width = screen_size.x;
        let height = screen_size.y;

        let item_spacing = 5.0;
        let margin = 10.0;

        let button_width = ((width - margin - item_spacing) / ((cols as f32) + 3.0)) - item_spacing;
        let button_height = ((height - margin - item_spacing) / ((rows as f32) + 2.0)) - item_spacing;

        let button_size = [button_width, button_height];

        Self { item_spacing, margin, width, height, button_size, rows, cols }
    }
}

pub struct App {
    speech_engine: SpeechEngine,
    panel: Panel,
    system: System,
    current_folder: usize,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let speech_engine = SpeechEngine::default().expect("Failed to initialize text-to-speech system");
        let panel = Panel::default();

        //let system: System = System::load_file("system.json").expect("Failed to load System from ./system.json");
        let system: System = System::load_str(include_str!("../system.json")).expect("Failed to load System from bundled system.json");
        let current_folder = 0;

        Self { speech_engine, panel, system, current_folder }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.input(|i| i.screen_rect().max);
        let folder = &self.system.folders[self.current_folder];
        let dimensions = Dimensions::new(screen_size, folder.rows, folder.cols);
        info!("{:?}", dimensions);

        let inner_margin = egui::style::Margin::same(dimensions.margin);
        ctx.style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(dimensions.item_spacing, dimensions.item_spacing);
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
                    if self.speech_engine.is_speaking() {
                        let egui_button = egui::Button::new("Stop");
                        if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                            self.speech_engine.stop();
                        }
                    } else {
                        let egui_button = egui::Button::new("Speak");
                        if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                            self.speech_engine.speak(self.panel.get_pronouncible_text()).unwrap();
                            self.panel.clear();
                        }
                    }
                });

                // Row 1, Column 2
                egui::Grid::new("top-center").show(ui, |ui| {
                    let cols = self.system.folders[self.current_folder].cols;
                    let inner_spacing = ui.ctx().style().spacing.item_spacing[0];
                    let max_width = (cols as f32) * (dimensions.button_size[0] + inner_spacing);

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
                                let _ = ui.add_sized(dimensions.button_size, egui_button);
                            };
                        }
                    });
                });

                // Row 1, Column 3
                egui::Grid::new("top-right").show(ui, |ui| {
                    let egui_button = egui::Button::new("Delete");
                    if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                        self.panel.remove_last_entry();
                    }

                    let egui_button = egui::Button::new("Clear");
                    if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                        self.panel.clear();
                    }
                });

                ui.end_row();

                // Row 2, Column 1
                egui::Grid::new("folder-selector-grid").show(ui, |ui| {
                    for (idx, folder) in self.system.folders.iter().enumerate() {
                        let egui_button = egui::Button::new(folder.name.clone()).selected(self.current_folder == idx);
                        if ui.add_sized(dimensions.button_size, egui_button).clicked() {
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
                                if ui.add_sized(dimensions.button_size, egui_button).clicked() {
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
                                ui.add_sized(dimensions.button_size, egui_label);
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
