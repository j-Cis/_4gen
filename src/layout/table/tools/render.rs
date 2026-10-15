// src/layout/table/tools/render.rs

use eframe::egui;
use crate::layout::setup::build_table_toolbar;
use crate::layout::table::tools::engine::ToolbarItem;
use crate::layout::table::types::{TableAction, TableState};
use crate::theme::tab::TableTheme;

pub struct TableToolbar;

impl TableToolbar {
    pub fn show(ui: &mut egui::Ui, theme: &TableTheme, _state: &mut TableState) -> Option<TableAction> {
        let mut action_triggered = None;

        egui::Frame::default()
            .fill(theme.toolbar_bg)
            .inner_margin(4.0)
            .show(ui, |ui| {
                egui::ScrollArea::horizontal()
                    .id_salt("table_toolbar_overflow")
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 2.0;

                            for item in build_table_toolbar() {
                                match item {
                                    ToolbarItem::Button(btn) => {
                                        if ui.button(btn.label_or_icon).on_hover_text(btn.tooltip).clicked() {
                                            action_triggered = Some(btn.action);
                                        }
                                    }
                                    ToolbarItem::Separator => {
                                        ui.separator();
                                    }
                                    ToolbarItem::Spacer(w) => {
                                        ui.add_space(w);
                                    }
                                }
                            }
                        });
                    });
            });

        action_triggered
    }
}