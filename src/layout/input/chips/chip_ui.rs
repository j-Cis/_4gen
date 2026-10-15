// src/layout/input/chips/chip_ui.rs

use eframe::egui::{self, Color32, CursorIcon, Frame, Margin, Rect, Sense, Stroke, Vec2};
use crate::layout::input::chips::types::ChipItem;

pub struct ChipUiOutput {
    pub clicked: bool,
    pub add_param_clicked: bool,
    pub remove_clicked: bool,
    pub param_changed: bool,
    pub rect: Rect,
}

pub fn render_chip(ui: &mut egui::Ui, item: &mut ChipItem, selected: bool) -> ChipUiOutput {
    let mut remove_clicked = false;
    let mut add_param_clicked = false;
    let mut param_changed = false;

    ui.style_mut().interaction.selectable_labels = false;

    let is_editing = match item {
        ChipItem::Parametric { is_editing, .. } => *is_editing,
        _ => false,
    };

    let bg_color = if is_editing {
        Color32::from_rgb(26, 45, 68)
    } else if selected {
        Color32::from_rgb(38, 79, 120)
    } else {
        Color32::from_rgb(45, 48, 54)
    };

    let border_stroke = if is_editing {
        Stroke::new(1.5, Color32::from_rgb(0, 180, 255))
    } else if selected {
        Stroke::new(1.0, Color32::from_rgb(0, 150, 255))
    } else {
        Stroke::new(1.0, Color32::from_gray(70))
    };

    let frame = Frame::new()
        .fill(bg_color)
        .stroke(border_stroke)
        .corner_radius(12.0)
        .inner_margin(Margin {
            left: 8,
            right: 6,
            top: 2,
            bottom: 2,
        });

    let resp = frame.show(ui, |chip_ui| {
        chip_ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            match item {
                // 1. ZWYKŁY CHIP
                ChipItem::Simple { label, .. } => {
                    let lbl_resp = ui.add(
                        egui::Label::new(
                            egui::RichText::new(label.as_str()).color(Color32::WHITE).size(13.0),
                        )
                        .selectable(false)
                        .sense(Sense::click()),
                    );

                    if lbl_resp.clicked() {
                        add_param_clicked = true;
                    }
                    if lbl_resp.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }

                    // Przycisk [+] do dodania parametru
                    let (plus_rect, plus_resp) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
                    let plus_resp = plus_resp.on_hover_text("Dodaj parametr");
                    let plus_hover = plus_resp.hovered();

                    let plus_col = if plus_hover {
                        Color32::from_rgb(100, 200, 255)
                    } else {
                        Color32::from_gray(160)
                    };

                    let p_center = plus_rect.center();
                    ui.painter().line_segment(
                        [egui::pos2(p_center.x - 4.0, p_center.y), egui::pos2(p_center.x + 4.0, p_center.y)],
                        Stroke::new(1.5, plus_col),
                    );
                    ui.painter().line_segment(
                        [egui::pos2(p_center.x, p_center.y - 4.0), egui::pos2(p_center.x, p_center.y + 4.0)],
                        Stroke::new(1.5, plus_col),
                    );

                    if plus_resp.clicked() {
                        add_param_clicked = true;
                    }
                    if plus_hover {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }
                }

                // 2. CHIP Z PARAMETREM (slot inline)
                ChipItem::Parametric { id, key, slot, is_editing } => {
                    let key_resp = ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("{}:", key))
                                .color(Color32::from_rgb(170, 190, 220))
                                .size(13.0)
                                .strong(),
                        )
                        .selectable(false)
                        .sense(Sense::click()),
                    );

                    if key_resp.clicked() {
                        *is_editing = !*is_editing;
                    }
                    if key_resp.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }

                    let slot_id = format!("{}_slot", id);
                    if slot.render_inline(ui, &slot_id, is_editing) {
                        param_changed = true;
                    }
                }
            }

            // Przycisk [✕]
            let (x_rect, x_resp) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::click());
            let x_resp = x_resp.on_hover_text("Usuń");

            let x_col = if x_resp.hovered() {
                Color32::from_rgb(255, 100, 100)
            } else {
                Color32::from_gray(160)
            };

            let pad = 3.5;
            ui.painter().line_segment(
                [x_rect.min + Vec2::splat(pad), x_rect.max - Vec2::splat(pad)],
                Stroke::new(1.2, x_col),
            );
            ui.painter().line_segment(
                [
                    egui::pos2(x_rect.max.x - pad, x_rect.min.y + pad),
                    egui::pos2(x_rect.min.x + pad, x_rect.max.y - pad),
                ],
                Stroke::new(1.2, x_col),
            );

            if x_resp.clicked() {
                remove_clicked = true;
            }
            if x_resp.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
        });
    });

    let chip_response = resp.response;

    ChipUiOutput {
        clicked: chip_response.clicked() && !remove_clicked && !add_param_clicked,
        add_param_clicked,
        remove_clicked,
        param_changed,
        rect: chip_response.rect,
    }
}