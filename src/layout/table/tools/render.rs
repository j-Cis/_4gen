// src/layout/table/tools/render.rs

use eframe::egui;
use crate::layout::setup::build_table_toolbar;
use crate::layout::table::tools::engine::{self, ToolbarItem};
use crate::layout::table::types::{TableAction, TableState};
use crate::theme::tab::TableTheme;

pub struct TableToolbar;

impl TableToolbar {
    pub fn show(ui: &mut egui::Ui, theme: &TableTheme, state: &mut TableState) -> Option<TableAction> {
        let mut action_triggered = None;

        // Odczytujemy stan aktywności modyfikatorów uwzględniający zarówno
        // wirtualne kliknięcie (simulated), jak i fizyczne trzymanie klawisza na klawiaturze
        let (shift_active, ctrl_active, alt_active) = ui.input(|i| (
            state.is_shift_active(i),
            state.is_ctrl_active(i),
            state.is_alt_active(i),
        ));

        egui::Frame::default()
            .fill(theme.toolbar_bg)
            .inner_margin(4.0)
            .show(ui, |ui| {
                // OBSŁUGA OVERFLOW: poziomy scroller bez paska przewijania na wąskich ekranach
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
                                    ToolbarItem::Modifier(m) => {
                                        let is_active = match m.action {
                                            TableAction::ToggleShift => shift_active,
                                            TableAction::ToggleCtrl => ctrl_active,
                                            TableAction::ToggleAlt => alt_active,
                                            _ => false,
                                        };

                                        // Podświetlenie aktywuje się natychmiast przy wciśnięciu klawisza fizycznego
                                        let btn = egui::Button::new(m.label).selected(is_active);

                                        if ui.add(btn).on_hover_text(m.tooltip).clicked() {
                                            match m.action {
                                                TableAction::ToggleShift => engine::toggle_shift(state),
                                                TableAction::ToggleCtrl => engine::toggle_ctrl(state),
                                                TableAction::ToggleAlt => engine::toggle_alt(state),
                                                _ => {}
                                            }
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