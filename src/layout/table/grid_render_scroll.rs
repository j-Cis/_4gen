// src/layout/table/grid_render_scroll.rs

use eframe::egui::{self, Color32, Rect, Sense, UiBuilder, vec2};
use crate::layout::table::types::{RowPinPosition, TableConfig, TableState};
use crate::theme::tab::TableTheme;

pub fn render_scroll_body<F>(
    ui: &mut egui::Ui,
    col_ids: &[String],
    state: &mut TableState,
    config: &TableConfig,
    theme: &TableTheme,
    row_indices: &[usize],
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    cell_renderer: &mut F,
) where
    F: FnMut(usize, &str, &mut egui::Ui),
{
    for (display_idx, &true_idx) in row_indices.iter().enumerate() {
        let is_selected = state.selected_rows.contains(&true_idx);
        let pin_pos = state.row_pin(true_idx);
        let row_bg = if is_selected {
            theme.row_selected_bg
        } else if display_idx % 2 == 0 {
            theme.row_even_bg
        } else {
            theme.row_odd_bg
        };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for (col_idx, col_id) in col_ids.iter().enumerate() {
                let width_px = state.columns.get(col_id).map(|c| c.width_px).unwrap_or(100.0);
                let (rect, mut response) = ui.allocate_exact_size(
                    vec2(width_px, theme.row_height),
                    Sense::click(),
                );

                if response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    if shift_pressed {
                        let msg = if pin_pos == RowPinPosition::Top {
                            "Kliknij, aby odpiąć wiersz z góry"
                        } else {
                            "Kliknij, aby przypiąć wiersz u góry"
                        };
                        response = response.on_hover_text(msg);
                    } else if alt_pressed {
                        let msg = if pin_pos == RowPinPosition::Bottom {
                            "Kliknij, aby odpiąć wiersz z dołu"
                        } else {
                            "Kliknij, aby przypiąć wiersz u dołu"
                        };
                        response = response.on_hover_text(msg);
                    }
                }

                ui.painter().rect_filled(rect, 0.0, row_bg);

                // Subtelny pasek akcentujący przypięcie na pierwszej kolumnie
                if col_idx == 0 && pin_pos != RowPinPosition::None {
                    let accent_color = match pin_pos {
                        RowPinPosition::Top => Color32::from_rgb(0, 150, 255),
                        RowPinPosition::Bottom => Color32::from_rgb(255, 170, 0),
                        _ => Color32::TRANSPARENT,
                    };
                    ui.painter().rect_filled(
                        Rect::from_min_size(rect.left_top(), vec2(3.0, theme.row_height)),
                        0.0,
                        accent_color,
                    );
                }

                let mut cell_ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(rect.shrink(4.0))
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                cell_ui.style_mut().interaction.selectable_labels = false;
                cell_renderer(true_idx, col_id, &mut cell_ui);

                if response.clicked() {
                    if shift_pressed {
                        state.toggle_pin_row_top(true_idx);
                    } else if alt_pressed {
                        state.toggle_pin_row_bottom(true_idx);
                    } else {
                        state.toggle_selection(true_idx, config, ctrl_pressed);
                    }
                }

                ui.painter().line_segment(
                    [rect.right_top(), rect.right_bottom()],
                    egui::Stroke::new(1.0, Color32::from_gray(35)),
                );
            }
        });
    }
}