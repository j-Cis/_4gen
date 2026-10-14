use eframe::egui::{self, Frame};

pub struct ContainerCanvas;

impl ContainerCanvas {
    pub fn show<F>(ui: &mut egui::Ui, add_contents: F)
    where
        F: FnOnce(&mut egui::Ui),
    {
        Frame::new()
            .inner_margin(12)
            .show(ui, |ui| {
                add_contents(ui);
            });
    }
}