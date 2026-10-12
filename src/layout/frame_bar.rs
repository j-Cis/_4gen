// src\layout\frame_bar.rs

use crate::theme::{hue, ico};
use eframe::egui::{self, Align, Align2, CursorIcon, Image, Layout, Sense, Ui, Vec2};

#[derive(Clone, Copy, PartialEq)]
pub enum FrameBarPlacement {
    Left,
    Right,
}

#[derive(Clone)]
pub enum FrameBarItem<'a> {
    Action {
        id: &'static str,
        icon: Image<'a>,
        tooltip: &'static str,
    },
    Separator,
}

pub struct FrameBar<'a> {
    pub placement: FrameBarPlacement,
    pub items: Vec<FrameBarItem<'a>>,
}

impl<'a> FrameBar<'a> {
    pub fn left(items: Vec<FrameBarItem<'a>>) -> Self {
        Self {
            placement: FrameBarPlacement::Left,
            items,
        }
    }

    pub fn right(items: Vec<FrameBarItem<'a>>) -> Self {
        Self {
            placement: FrameBarPlacement::Right,
            items,
        }
    }

    pub fn show(&self, ui: &mut Ui) -> Vec<&'static str> {
        let mut activated = Vec::new();

        ui.spacing_mut().item_spacing =
            egui::vec2(ico::ICON_PADDING_MAIN_AXIS, ico::ICON_PADDING_MAIN_AXIS);

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            for item in &self.items {
                self.show_item(ui, item, &mut activated);
            }
        });

        activated
    }

    fn show_item(&self, ui: &mut Ui, item: &FrameBarItem, activated: &mut Vec<&'static str>) {
        match item {
            FrameBarItem::Separator => {
                ui.add_space(ico::ICON_PADDING_MAIN_AXIS);
                ui.separator();
                ui.add_space(ico::ICON_PADDING_MAIN_AXIS);
            }
            FrameBarItem::Action { id, icon, tooltip } => {
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
                let icon_rect = egui::Rect::from_center_size(rect.center(), ico::ICON_SIZE_VEC);
                icon.clone()
                    .fit_to_exact_size(ico::ICON_SIZE_VEC)
                    .paint_at(ui, icon_rect);

                if response.hovered() && !tooltip.is_empty() {
                    let (tooltip_pos, tooltip_pivot) = match self.placement {
                        FrameBarPlacement::Left => (
                            egui::pos2(rect.right() + 6.0, rect.center().y),
                            Align2::LEFT_CENTER,
                        ),
                        FrameBarPlacement::Right => (
                            egui::pos2(rect.left() - 6.0, rect.center().y),
                            Align2::RIGHT_CENTER,
                        ),
                    };

                    egui::Area::new(ui.id().with(id).with("tooltip"))
                        .fixed_pos(tooltip_pos)
                        .pivot(tooltip_pivot)
                        .order(egui::Order::Tooltip)
                        .interactable(false)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::popup(ui.style()).show(ui, |ui| {
                                ui.label(*tooltip);
                            });
                        });
                }

                if response.clicked() {
                    activated.push(*id);
                }
            }
        }
    }
}
