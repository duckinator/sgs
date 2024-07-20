use log::{info, warn}; // also available: trace.
use eframe::egui;
use eframe::egui::Pos2;

use crate::system::System;
use crate::panel::Panel;
use crate::speech::SpeechEngine;
use crate::cli;

use std::cmp;

pub const MIN_WIDTH: f32 = 1280.0;
pub const MIN_HEIGHT: f32 = 720.0;

#[derive(Debug)]
struct Dimensions {
    item_spacing: f32,
    margin: f32,
    button_size: [f32; 2],
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

        Self { item_spacing, margin, button_size }
    }
}

pub struct App {
    speech_engine: SpeechEngine,
    panel: Panel,
    system: System,
    current_folder: usize,
    current_page: usize,
    current_hotbar_page: usize,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let speech_engine = SpeechEngine::default().expect("Failed to initialize text-to-speech system");
        let panel = Panel::default();

        let args = cli::parse_args().unwrap();

        let system: System =
            if let Some(system_file) = args.system {
                System::load_file(&system_file).expect("Failed to load System from specified file")
            } else {
                System::load_str(include_str!("../system-wiki2016.json")).expect("Failed to load bundled System. (This should never happen.)")
            };
        info!("Loaded System configuration.");

        let current_folder = 0;
        let current_page = 0;
        let current_hotbar_page = 0;

        Self { speech_engine, panel, system, current_folder, current_page, current_hotbar_page }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.input(|i| i.screen_rect().max);
        let hotbar = &self.system.hotbar;
        let folder = &self.system.folders[self.current_folder];
        let dimensions = Dimensions::new(screen_size, folder.rows, folder.cols);

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
                            self.speech_engine.speak(self.panel.get_pronouncible_text(&self.system)).unwrap();
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
                            let egui_button = egui::Button::new(entry.get_label(&self.system).clone());
                            if self.panel.entries.len() > cols {
                                let _ = ui.add(egui_button);
                            } else {
                                let _ = ui.add_sized(dimensions.button_size, egui_button);
                            };
                        }
                    });
                });

                // Row 1, Column 3
                egui::Grid::new("delete-btn").show(ui, |ui| {
                    let egui_button = egui::Button::new("Delete");
                    if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                        self.panel.remove_last_entry();
                    }
                });

                egui::Grid::new("clear-btn").show(ui, |ui| {
                    let egui_button = egui::Button::new("Clear");
                    if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                        self.panel.clear();
                    }
                });

                ui.end_row();

                // Row 2, Column 1
                egui::Grid::new("folder-selector-grid").show(ui, |ui| {
                    let current_toplevel_folder = self.system.toplevel_folder_for(self.current_folder);
                    for (idx, folder) in self.system.toplevel_folders().iter().enumerate() {
                        let egui_button = egui::Button::new(folder.name.clone()).selected(current_toplevel_folder == idx);
                        if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                            self.current_page = 0;
                            self.current_folder = idx;
                        }
                        ui.end_row();
                    }
                });

                // Row 2, Column 2
                egui::Grid::new("active-folder").show(ui, |ui| {
                    for row in 0..folder.rows {
                        for col in 0..folder.cols {
                            if col == (folder.cols - 1) && row == (folder.rows - 1) && folder.needs_pagination() {
                                let num_buttons = folder.buttons.len();
                                let pages = num_buttons / (folder.rows * folder.cols);

                                // Account for the last page being only partially filled.
                                let pages =
                                    if (pages * ((folder.rows * folder.cols) - 1)) < num_buttons {
                                        pages + 1
                                    } else {
                                        pages
                                    };

                                let label = format!("{}/{}\n->", self.current_page + 1, pages);
                                let egui_button = egui::Button::new(label);
                                if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                                    self.current_page = folder.next_page(self.current_page);
                                }
                                break;
                            }


                            if let Some(button) = folder.get_button(self.current_page, col, row) {
                                let egui_button = egui::Button::new(button.label.clone());
                                if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                                    if let Some(btn_folder) = &button.folder {
                                        let index = self.system.folders.iter().position(|x| &x.id == btn_folder);
                                        if let Some(index) = index {
                                            self.current_folder = index;
                                        } else {
                                            warn!("!!! Tried to open folder named '{}', which does not exist.", btn_folder);
                                        }
                                    } else if folder.immediate {
                                        self.speech_engine.speak(button.get_pronouncible_text(&self.system)).expect("Failed to speak word");
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

                    for col in 0..folder.cols {
                        if col == (folder.cols - 1) {
                            let label = format!("{}\n->", self.current_hotbar_page + 1);
                            let egui_button = egui::Button::new(label);
                            if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                                self.current_hotbar_page = self.system.hotbar.next_page(folder.cols, self.current_hotbar_page);
                            }
                            break;
                        }

                        if let Some(button) = hotbar.get_button(folder.cols, self.current_hotbar_page, col) {
                            let egui_button = egui::Button::new(button.label.clone());
                            if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                                self.panel.add_entry(button);
                            }
                        } else {
                            // No button for this position on the hotbar.
                            let egui_label = egui::Label::new("");
                            //let egui_label = egui::Button::new("");
                            ui.add_sized(dimensions.button_size, egui_label);
                        }
                    }
                    ui.end_row();
                });

                // Row 2, Column 3
                egui::Grid::new("related-words").show(ui, |ui| {
                    if let Some(last_word) = self.panel.last_entry_label() {
                        if let Some(related) = self.system.related.get(&last_word) {
                            for row in 0..cmp::min(folder.rows, related.len()) {
                                let related_idx = row;

                                let button = &related[row];
                                let egui_button = egui::Button::new(button.get_label(&self.system).clone());
                                if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                                    self.panel.set_last_entry_related(related_idx);
                                    self.panel.clear_last_entry_variant();
                                }
                                ui.end_row();
                            }
                        }
                    }
                });

                // Row 2, Column 4
                egui::Grid::new("variant-words").show(ui, |ui| {
                    if let Some(last_word) = self.panel.last_entry_related_label(&self.system) {
                        if let Some(variants) = self.system.variants.get(&last_word) {
                            for row in 0..cmp::min(folder.rows, variants.len()) {
                                let variant = row;

                                let button = &variants[variant];
                                let egui_button = egui::Button::new(button.get_label(&self.system).clone());
                                if ui.add_sized(dimensions.button_size, egui_button).clicked() {
                                    self.panel.set_last_entry_variant(variant);
                                }
                                ui.end_row();
                            }
                        }
                    }
                });
            });
        });
    }
}
