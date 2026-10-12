// src/layout/table_toolbar.rs

use eframe::egui;
use crate::theme::tab::{TableIcons, TableTheme};
use crate::layout::table_types::TableAction;

pub struct TableToolbar;

impl TableToolbar {
    pub fn show(ui: &mut egui::Ui, theme: &TableTheme) -> Option<TableAction> {
        let mut action_triggered = None;

        egui::Frame::default()
            .fill(theme.toolbar_bg)
            .inner_margin(4.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;

                    if ui.button(TableIcons::PREV).on_hover_text("Poprzednie zaznaczenie").clicked() {
                        action_triggered = Some(TableAction::Prev);
                    }
                    if ui.button(TableIcons::NEXT).on_hover_text("Następne zaznaczenie").clicked() {
                        action_triggered = Some(TableAction::Next);
                    }

                    ui.add_space(8.0);
                    if ui.button(TableIcons::ADD).on_hover_text("Nowa pozycja").clicked() {
                        action_triggered = Some(TableAction::Add);
                    }
                    if ui.button(TableIcons::EDIT).on_hover_text("Edytuj pozycję").clicked() {
                        action_triggered = Some(TableAction::Edit);
                    }
                    if ui.button(TableIcons::DELETE).on_hover_text("Usuń pozycję").clicked() {
                        action_triggered = Some(TableAction::Delete);
                    }

                    ui.add_space(8.0);
                    if ui.button(TableIcons::MERGE).on_hover_text("Scal wybrane").clicked() {
                        action_triggered = Some(TableAction::Merge);
                    }
                    if ui.button(TableIcons::DUPLICATE).on_hover_text("Duplikuj pozycję").clicked() {
                        action_triggered = Some(TableAction::Duplicate);
                    }

                    ui.add_space(8.0);
                    if ui.button(TableIcons::TAG).on_hover_text("Etykietuj pozycję").clicked() {
                        action_triggered = Some(TableAction::Tag);
                    }

                    ui.add_space(8.0);
                    if ui.button(TableIcons::CONFIG).on_hover_text("Konfiguruj kolumny").clicked() {
                        action_triggered = Some(TableAction::ConfigColumns);
                    }
                });
            });

        action_triggered
    }
}