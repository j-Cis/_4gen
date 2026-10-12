// src/layout/table_grid_engine.rs

use eframe::egui::{self, Color32, Rect, Sense, UiBuilder, vec2};
use crate::theme::tab::TableTheme;
use crate::layout::table_types::{PinPosition, SortDir, TableConfig, TableState};

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

        let shift_pressed = ui.input(|i| i.modifiers.shift);
        let ctrl_pressed = ui.input(|i| i.modifiers.command);

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
                        Self::render_header_row(ui, &left_cols, state, theme, shift_pressed, true);
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
                        Self::render_header_row(ui, &center_cols, state, theme, shift_pressed, false);
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
                                Self::render_body_rows(
                                    ui, &left_cols, state, config, theme, row_indices, ctrl_pressed, &mut cell_renderer
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
                                    Self::render_body_rows(
                                        ui, &center_cols, state, config, theme, row_indices, ctrl_pressed, &mut cell_renderer
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

    fn render_header_row(
        ui: &mut egui::Ui,
        col_ids: &[String],
        state: &mut TableState,
        theme: &TableTheme,
        shift_pressed: bool,
        is_pinned: bool,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for col_id in col_ids {
                let (title, sortable, width_px) = match state.columns.get(col_id) {
                    Some(c) => (c.title.clone(), c.sortable, c.width_px),
                    None => continue,
                };

                let (rect, response) = ui.allocate_exact_size(
                    vec2(width_px, theme.header_height),
                    Sense::click_and_drag(),
                );

                ui.painter().rect_filled(rect, 0.0, theme.header_bg);

                let mut display_title = if is_pinned { format!("📌 {}", title) } else { title };
                if let Some((idx, (_, dir))) = state
                    .sort_rules
                    .iter()
                    .enumerate()
                    .find(|(_, (id, _))| id == col_id)
                {
                    let arrow = match dir {
                        SortDir::Asc => " 🔼",
                        SortDir::Desc => " 🔽",
                    };
                    let priority = if state.sort_rules.len() > 1 {
                        format!(" ({})", idx + 1)
                    } else {
                        "".to_string()
                    };
                    display_title = format!("{}{}{}", display_title, arrow, priority);
                }

                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    display_title,
                    egui::FontId::proportional(13.0),
                    Color32::WHITE,
                );

                if sortable && response.clicked() {
                    state.handle_sort_click(col_id, shift_pressed);
                }

                let resize_rect = Rect::from_min_max(
                    rect.right_top() - vec2(4.0, 0.0),
                    rect.right_bottom() + vec2(4.0, 0.0),
                );
                let resize_resp = ui.interact(
                    resize_rect,
                    ui.make_persistent_id(format!("resize_{}", col_id)),
                    Sense::drag(),
                );

                if resize_resp.dragged() {
                    let delta = resize_resp.drag_delta().x;
                    if let Some(col_def) = state.columns.get_mut(col_id) {
                        col_def.width_px = (col_def.width_px + delta).max(40.0);
                    }
                }

                if resize_resp.hovered() || resize_resp.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
                }

                ui.painter().line_segment(
                    [rect.right_top(), rect.right_bottom()],
                    egui::Stroke::new(1.0, Color32::from_gray(60)),
                );
            }
        });
    }

    fn render_body_rows<F>(
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
}