// src/layout/table/grid_engine.rs

use eframe::egui::{self, vec2};
use crate::layout::table::grid_render::{header, pinned, scroll};
use crate::layout::table::types::{PinPosition, TableConfig, TableState};
use crate::theme::tab::TableTheme;

pub struct TableGridEngine;

impl TableGridEngine {
    pub fn render<F>(
        ui: &mut egui::Ui,
        state: &mut TableState,
        config: &TableConfig,
        theme: &TableTheme,
        row_indices: &[usize],
        mut cell_renderer: F,
    ) where
        F: FnMut(usize, &str, &mut egui::Ui),
    {
        let active_order = state.get_computed_columns_order();

        let mut left_cols = Vec::new();
        let mut center_cols = Vec::new();

        for id in &active_order {
            if let Some(col) = state.columns.get(id) {
                match col.pinned {
                    PinPosition::Left => left_cols.push(id.clone()),
                    PinPosition::None => center_cols.push(id.clone()),
                }
            }
        }

        let shift_pressed = ui.input(|i| state.is_shift_active(i));
        let ctrl_pressed = ui.input(|i| state.is_ctrl_active(i));

        let left_total_width: f32 = left_cols
            .iter()
            .map(|id| state.columns.get(id).map(|c| c.width_px).unwrap_or(100.0))
            .sum();

        let available_height = ui.available_height();

        // Identyfikator w pamięci egui do synchronizacji przesunięcia w poziomie (X)
        let scroll_x_id = ui.id().with("table_horizontal_scroll_sync");
        let mut scroll_x = ui.data(|d| d.get_temp::<f32>(scroll_x_id).unwrap_or(0.0));

        // ==========================================
        // 1. NAGŁÓWKI (ZAMROŻONE W PIONIE)
        // ==========================================
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            if !left_cols.is_empty() {
                ui.allocate_ui_with_layout(
                    vec2(left_total_width, theme.header_height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        header::render_header_row(ui, &left_cols, state, theme, shift_pressed, true);
                    },
                );
                ui.separator();
            }

            if !center_cols.is_empty() {
                let header_scroll = egui::ScrollArea::horizontal()
                    .id_salt("header_hscroll")
                    .horizontal_scroll_offset(scroll_x)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        header::render_header_row(ui, &center_cols, state, theme, shift_pressed, false);
                    });

                // Aktualizujemy globalny offset jeśli nagłówek został przeciągnięty/przewinięty
                if header_scroll.state.offset.x != scroll_x {
                    scroll_x = header_scroll.state.offset.x;
                }
            }
        });

        // ==========================================
        // 2. CIAŁO TABELI (PRZEWIJANE W PIONIE)
        // ==========================================
        egui::ScrollArea::vertical()
            .id_salt("grid_vertical_scroll")
            .max_height(available_height - theme.header_height)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    if !left_cols.is_empty() {
                        ui.allocate_ui_with_layout(
                            vec2(left_total_width, 0.0), // 0.0 = pozwól rosnąć do dołu
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                pinned::render_pinned_body(
                                    ui, &left_cols, state, config, theme, row_indices, ctrl_pressed, &mut cell_renderer,
                                );
                            },
                        );
                        ui.separator();
                    }

                    if !center_cols.is_empty() {
                        let body_scroll = egui::ScrollArea::horizontal()
                            .id_salt("body_hscroll")
                            .horizontal_scroll_offset(scroll_x)
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    scroll::render_scroll_body(
                                        ui, &center_cols, state, config, theme, row_indices, ctrl_pressed, &mut cell_renderer,
                                    );
                                });
                            });

                        // Aktualizujemy globalny offset jeśli dolny pasek przewijania ruszył
                        if body_scroll.state.offset.x != scroll_x {
                            scroll_x = body_scroll.state.offset.x;
                        }
                    }
                });
            });

        // Zapisujemy nowy stan scroll_x dla następnej klatki
        ui.data_mut(|d| d.insert_temp(scroll_x_id, scroll_x));
    }
}