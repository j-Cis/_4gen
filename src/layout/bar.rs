// src\layout\bar.rs

use crate::theme::{hue, ico};
use eframe::egui::{Align, CursorIcon, Image, Layout, Sense, Ui, Vec2};

#[derive(Clone)]
pub enum BarItem<'a> {
    Action {
        id: &'static str,
        icon: Image<'a>,
        tooltip: &'static str,
    },
    Separator,
}

pub struct Bar<'a> {
    pub vertical: bool,
    pub items: Vec<BarItem<'a>>,
}

impl<'a> Bar<'a> {
    pub fn vertical(items: Vec<BarItem<'a>>) -> Self {
        Self {
            vertical: true,
            items,
        }
    }

    pub fn horizontal(items: Vec<BarItem<'a>>) -> Self {
        Self {
            vertical: false,
            items,
        }
    }

    pub fn show(&self, ui: &mut Ui) -> Vec<&'static str> {
        let mut activated = Vec::new();

        ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);

        if self.vertical {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                for item in &self.items {
                    self.show_item(ui, item, &mut activated);
                }
            });
        } else {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                for item in &self.items {
                    self.show_item(ui, item, &mut activated);
                }
            });
        }

        activated
    }

    fn show_item(&self, ui: &mut Ui, item: &BarItem, activated: &mut Vec<&'static str>) {
        match item {
            BarItem::Separator => {
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);
            }
            BarItem::Action { id, icon, tooltip } => {
                let (rect, mut response) =
                    ui.allocate_exact_size(Vec2::splat(ico::ICON_BUTTON_SIZE), Sense::click());

                response = response.on_hover_cursor(CursorIcon::PointingHand);

                let bg_color = if response.is_pointer_button_down_on() {
                    hue::MAIN_TOOLBAR_ACTIVE
                } else if response.hovered() {
                    hue::MAIN_TOOLBAR_HOVER
                } else {
                    hue::MAIN_TOOLBAR_BG
                };

                ui.painter()
                    .rect_filled(rect, ico::ICON_BUTTON_CORNER_RADIUS, bg_color);
                let icon_rect =
                    eframe::egui::Rect::from_center_size(rect.center(), ico::ICON_SIZE_VEC);

                icon.clone()
                    .fit_to_exact_size(ico::ICON_SIZE_VEC)
                    .paint_at(ui, icon_rect);

                // Klasyczny tooltip dla prostych toolbarów
                if !tooltip.is_empty() {
                    response = response.on_hover_text(*tooltip);
                }

                if response.clicked() {
                    activated.push(*id);
                }
            }
        }
    }
}
