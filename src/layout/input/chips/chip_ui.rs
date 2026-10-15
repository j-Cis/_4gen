// src/layout/input/chips/chip_ui.rs

use eframe::egui::{self, Color32, CursorIcon, Frame, Margin, Rect, Sense, Stroke, Vec2};
use crate::layout::input::chips::types::ChipItem;

pub struct ChipUiOutput {
    pub clicked: bool,
    pub param_clicked: bool,
    pub remove_clicked: bool,
    pub rect: Rect,
}

pub fn render_chip(ui: &mut egui::Ui, item: &ChipItem, selected: bool) -> ChipUiOutput {
    let mut remove_clicked = false;
    let mut param_clicked = false;

    ui.style_mut().interaction.selectable_labels = false;

    let bg_color = if selected {
        Color32::from_rgb(38, 79, 120)
    } else {
        Color32::from_rgb(45, 48, 54)
    };

    let border_stroke = if selected {
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
                // 1. ZWYKŁY CHIP (np. "Płeć" z przyciskiem "+")
                ChipItem::Simple { label, .. } => {
                    let lbl_resp = ui.add(
                        egui::Label::new(
                            egui::RichText::new(label).color(Color32::WHITE).size(13.0),
                        )
                        .selectable(false)
                        .sense(Sense::click()),
                    );

                    if lbl_resp.clicked() {
                        param_clicked = true;
                    }
                    if lbl_resp.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }

                    // Przycisk [+] do dodania parametru
                    let (plus_rect, plus_resp) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::click());
                    let plus_resp = plus_resp.on_hover_text("Dodaj parametr");

                    let plus_col = if plus_resp.hovered() {
                        Color32::from_rgb(100, 200, 255)
                    } else {
                        Color32::from_gray(160)
                    };

                    let p_center = plus_rect.center();
                    ui.painter().line_segment(
                        [egui::pos2(p_center.x - 3.5, p_center.y), egui::pos2(p_center.x + 3.5, p_center.y)],
                        Stroke::new(1.3, plus_col),
                    );
                    ui.painter().line_segment(
                        [egui::pos2(p_center.x, p_center.y - 3.5), egui::pos2(p_center.x, p_center.y + 3.5)],
                        Stroke::new(1.3, plus_col),
                    );

                    if plus_resp.clicked() {
                        param_clicked = true;
                    }
                    if plus_resp.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }
                }

                // 2. CHIP Z PARAMETREM (np. "Płeć: Kobieta")
                ChipItem::Parametric { key, display_value, .. } => {
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

                    let val_text = if display_value.is_empty() {
                        "<?>"
                    } else {
                        display_value.as_str()
                    };

                    let val_resp = ui
                        .add(
                            egui::Label::new(
                                egui::RichText::new(val_text)
                                    .color(if display_value.is_empty() {
                                        Color32::from_rgb(255, 180, 0)
                                    } else {
                                        Color32::WHITE
                                    })
                                    .size(13.0),
                            )
                            .selectable(false)
                            .sense(Sense::click()),
                        )
                        .on_hover_text("Kliknij, aby zmienić parametr");

                    // Kliknięcie w klucz lub w wartość wyzwala edycję
                    if key_resp.clicked() || val_resp.clicked() {
                        param_clicked = true;
                    }
                    if key_resp.hovered() || val_resp.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }
                }
            }

            // Przycisk [✕] do usunięcia chipa
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
        clicked: chip_response.clicked() && !remove_clicked && !param_clicked,
        param_clicked,
        remove_clicked,
        rect: chip_response.rect,
    }
}