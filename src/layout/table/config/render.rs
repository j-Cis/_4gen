// src/layout/table/config/render.rs

use eframe::egui::{self, Color32, Frame, Id, Stroke, vec2};
use crate::layout::frame::{ContainerWindow, ContainerWindowMode};
use crate::layout::table::config::engine;
use crate::layout::table::types::{PinPosition, TableState};
use crate::theme::tab::TableIcons;

pub struct TableConfigWindow {
    pub window: ContainerWindow,
}

impl Default for TableConfigWindow {
    fn default() -> Self {
        Self {
            window: ContainerWindow::new(
                "table_col_config",
                "Konfiguracja Kolumn",
                TableIcons::CONFIG,
                ContainerWindowMode::Floating,
            ),
        }
    }
}

impl TableConfigWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut TableState) {
        self.window.show(ui, |ui| {
            ui.label("Zaznacz widoczność, przypięcie (Początek/Koniec) oraz kolejność:");
            ui.add_space(8.0);

            let mut from_idx: Option<usize> = None;
            let mut to_idx: Option<usize> = None;

            let frame = Frame::default().inner_margin(4.0);

            ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
                ui.set_min_size(vec2(320.0, 100.0));

                let display_order = state.display_order.clone();

                for (row_idx, col_id) in display_order.iter().enumerate() {
                    let item_id = Id::new("col_dnd_item").with(row_idx);

                    ui.horizontal(|ui| {
                        // 1. CHECKBOX WIDOCZNOŚCI
                        if state.columns.contains_key(col_id) {
                            let mut is_visible = state.visible_columns.contains(col_id);
                            if ui.checkbox(&mut is_visible, "").changed() {
                                engine::toggle_column_visibility(state, col_id, is_visible);
                            }
                        }

                        // 2. UCHWYT DRAG & DROP Z NAZWĄ
                        let handle_response = ui
                            .dnd_drag_source(item_id, row_idx, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("☰");
                                    if let Some(col) = state.columns.get(col_id) {
                                        ui.label(&col.title);
                                    }
                                });
                            })
                            .response;

                        if let (Some(pointer), Some(hovered_payload)) = (
                            ui.input(|i| i.pointer.interact_pos()),
                            handle_response.dnd_hover_payload::<usize>(),
                        ) {
                            let rect = handle_response.rect;
                            let stroke = Stroke::new(2.0, Color32::from_rgb(0, 120, 215));

                            let insert_target = if *hovered_payload == row_idx {
                                ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                                row_idx
                            } else if pointer.y < rect.center().y {
                                ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                row_idx
                            } else {
                                ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                row_idx + 1
                            };

                            if let Some(dragged_payload) = handle_response.dnd_release_payload() {
                                from_idx = Some(*dragged_payload);
                                to_idx = Some(insert_target);
                            }
                        }

                        // 3. SEKCJA PRAWA: PRZYPIĘCIE ORAZ PRZYCISKI GÓRA/DÓŁ
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Przycisk DÓŁ
                            if row_idx + 1 < state.display_order.len() {
                                if ui.button(TableIcons::DOWN).on_hover_text("Przesuń w dół").clicked() {
                                    from_idx = Some(row_idx);
                                    to_idx = Some(row_idx + 1);
                                }
                            }
                            // Przycisk GÓRA
                            if row_idx > 0 {
                                if ui.button(TableIcons::UP).on_hover_text("Przesuń w górę").clicked() {
                                    from_idx = Some(row_idx);
                                    to_idx = Some(row_idx - 1);
                                }
                            }

                            // PRZYCISKI PRZYPIĘCIA LEWO
                            if let Some(col) = state.columns.get(col_id) {
                                let is_left = col.pinned == PinPosition::Left;
                                if ui.selectable_label(is_left, "📌L").on_hover_text("Przypnij na początku (lewo)").clicked() {
                                    engine::toggle_pin_left(state, col_id);
                                }
                            }
                        });
                    });
                    ui.separator();
                }
            });

            if let (Some(from), Some(to)) = (from_idx, to_idx) {
                engine::reorder_columns(state, from, to);
            }
        });
    }
}