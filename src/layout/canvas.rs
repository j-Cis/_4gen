// src/layout/canvas.rs

use eframe::egui;
use crate::widget::{
    records_individuals::RecordsIndividualsWidget,
    records_assets::RecordsAssetsWidget,
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CanvasMode {
    Empty,
    Individuals,
    Assets,
}

pub struct WorkspaceCanvas {
    pub mode: CanvasMode,
    pub individuals_widget: RecordsIndividualsWidget,
}

impl Default for WorkspaceCanvas {
    fn default() -> Self {
        Self {
            mode: CanvasMode::Individuals,
            individuals_widget: RecordsIndividualsWidget::default(),
        }
    }
}

impl WorkspaceCanvas {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Frame::default()
            .inner_margin(16.0)
            .show(ui, |ui| {
                match self.mode {
                    CanvasMode::Empty => {
                        ui.centered_and_justified(|ui| {
                            ui.label(
                                egui::RichText::new("Wybierz widok z lewego paska")
                                    .size(18.0)
                                    .weak(),
                            );
                        });
                    }
                    CanvasMode::Individuals => {
                        self.individuals_widget.show(ui);
                    }
                    CanvasMode::Assets => {
                        RecordsAssetsWidget::show(ui);
                    }
                }
            });
    }
}