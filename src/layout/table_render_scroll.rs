// src/layout/table_render_scroll.rs

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use crate::theme::tab::TableTheme;
use crate::layout::table_types::{SortDir, TableConfig, TableState};

pub struct TableRenderScroll;

impl TableRenderScroll {
    pub fn render_section<F>(
        ui: &mut egui::Ui,
        section_id: &str,
        col_ids: &[String],
        state: &mut TableState,
        config: &TableConfig,
        theme: &TableTheme,
        row_count: usize,
        available_height: f32,
        mut cell_renderer: F,
    ) where
        F: FnMut(usize, &str, &mut egui::Ui),
    {
        if col_ids.is_empty() {
            return;
        }

        let shift_pressed = ui.input(|i| i.modifiers.shift);
        let ctrl_pressed = ui.input(|i| i.modifiers.command);

        egui::ScrollArea::horizontal()
            .id_salt(format!("{}_scroll", section_id))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let mut builder = TableBuilder::new(ui)
                    .id_salt(section_id)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .sense(egui::Sense::click())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                for col_id in col_ids {
                    if let Some(col_def) = state.columns.get(col_id) {
                        builder = builder.column(Column::initial(col_def.width_px).resizable(true));
                    }
                }

                builder
                    .header(theme.header_height, |mut header| {
                        for col_id in col_ids {
                            if let Some(col_def) = state.columns.get(col_id).cloned() {
                                header.col(|cell_ui| {
                                    cell_ui.painter().rect_filled(cell_ui.max_rect(), 0.0, theme.header_bg);
                                    cell_ui.horizontal_centered(|cell_ui| {
                                        let mut title_text = col_def.title.clone();

                                        if let Some((idx, (_, dir))) = state
                                            .sort_rules
                                            .iter()
                                            .enumerate()
                                            .find(|(_, (id, _))| id == &col_def.id)
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
                                            title_text = format!("{}{}{}", title_text, arrow, priority);
                                        }

                                        let text = egui::RichText::new(title_text)
                                            .strong()
                                            .color(egui::Color32::WHITE);
                                        let resp = cell_ui.selectable_label(false, text);
                                        if col_def.sortable && resp.clicked() {
                                            state.handle_sort_click(&col_def.id, shift_pressed);
                                        }
                                    });
                                });
                            }
                        }
                    })
                    .body(|body| {
                        body.rows(theme.row_height, row_count, |mut row| {
                            let row_idx = row.index();
                            let is_selected = state.selected_rows.contains(&row_idx);
                            row.set_selected(is_selected);

                            for col_id in col_ids {
                                row.col(|cell_ui| {
                                    cell_renderer(row_idx, col_id, cell_ui);
                                });
                            }

                            if row.response().clicked() {
                                state.toggle_selection(row_idx, config, ctrl_pressed);
                            }
                        });
                    });
            });
    }
}