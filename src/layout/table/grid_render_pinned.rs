// src/layout/table/grid_render_pinned.rs

use eframe::egui::{self, Color32, Sense, UiBuilder, vec2};
use crate::layout::table::types::{TableConfig, TableState};
use crate::theme::tab::TableTheme;

pub fn render_pinned_body<F>(
    ui: &mut egui::Ui,
    col_ids: &[String],
    state: &mut TableState,
    config: &TableConfig,
    theme: &TableTheme,
    row_indices: &[usize],
    ctrl_pressed: bool,
    cell_renderer: &mut F,
) where
    F: FnMut(usize, &str, &mut egui::Ui),
{
    for (display_idx, &true_idx) in row_indices.iter().enumerate() {
        let is_selected = state.selected_rows.contains(&true_idx);
        let row_bg = if is_selected {
            theme.row_selected_bg
        } else if display_idx % 2 == 0 {
            theme.row_even_bg
        } else {
            theme.row_odd_bg
        };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for col_id in col_ids {
                let width_px = state.columns.get(col_id).map(|c| c.width_px).unwrap_or(100.0);
                let (rect, response) = ui.allocate_exact_size(
                    vec2(width_px, theme.row_height),
                    Sense::click(),
                );

                ui.painter().rect_filled(rect, 0.0, row_bg);

                let mut cell_ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(rect.shrink(4.0))
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                cell_renderer(true_idx, col_id, &mut cell_ui);

                if response.clicked() {
                    state.toggle_selection(true_idx, config, ctrl_pressed);
                }

                ui.painter().line_segment(
                    [rect.right_top(), rect.right_bottom()],
                    egui::Stroke::new(1.0, Color32::from_gray(35)),
                );
            }
        });
    }
}