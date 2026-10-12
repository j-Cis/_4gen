// src/pipe/icon_button.rs

use eframe::egui::{self, Image, Response, Ui};

pub struct IconButton;

impl IconButton {
    /// Rysuje przycisk ikoniczny z obrazu `Image` i podpiętym podpowiedzią (tooltip)
    pub fn show(ui: &mut Ui, icon: Image<'_>, tooltip: &str) -> Response {
        let btn = egui::Button::image(icon);
        ui.add(btn).on_hover_text(tooltip)
    }

    /// Rysuje mały przycisk ikoniczny (np. do ciasnych list i narzędzi)
    pub fn show_small(ui: &mut Ui, icon: Image<'_>, tooltip: &str) -> Response {
        let btn = egui::Button::image(icon).min_size(egui::vec2(18.0, 18.0));
        ui.add(btn).on_hover_text(tooltip)
    }
}