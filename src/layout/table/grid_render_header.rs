use eframe::egui::{self, Color32, Rect, Sense, vec2};
use crate::layout::table::types::{SortDir, TableState};
use crate::theme::tab::TableTheme;

pub fn render_header_row(
    ui: &mut egui::Ui,
    col_ids: &[String],
    state: &mut TableState,
    theme: &TableTheme,
    shift_pressed: bool,
    ctrl_pressed: bool,
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
                Sense::click(),
            );

            // Kursor rączki nad nagłówkiem kolumny
            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

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

            let tooltip_text = if is_pinned {
                "Kliknij z Shift, aby odpiąć.\nKliknij z Ctrl, aby sortować po wielu."
            } else {
                "Kliknij z Shift, aby przypiąć.\nKliknij z Ctrl, aby sortować po wielu."
            };
            let response = response.on_hover_text(tooltip_text);

            if response.clicked() {
                if shift_pressed {
                    state.toggle_pin_column(col_id);
                } else if sortable {
                    state.handle_sort_click(col_id, ctrl_pressed);
                }
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