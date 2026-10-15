// src/layout/input/chips/chip_slot.rs

use eframe::egui;

/// Interfejs gniazda (slotu) parametru.
/// Pozwala na wstrzyknięcie dowolnego edytora: tekstowego, wyboru daty, koloru, dropdowna itp.
pub trait ChipParamSlot {
    /// Tekst prezentowany na pigułce w stanie spoczynku
    fn display_text(&self) -> String;

    /// Rysuje aktywny edytor wewnątrz popupu lub na klikniętym chipie.
    /// Zwraca true, jeśli wartość uległa zmianie.
    fn render_editor(&mut self, ui: &mut egui::Ui) -> bool;
}

/// Domyślna, prosta tekstowa implementacja slotu parametru
#[derive(Clone, Debug, Default)]
pub struct TextParamSlot {
    pub value: String,
}

impl TextParamSlot {
    pub fn new(val: impl Into<String>) -> Self {
        Self {
            value: val.into(),
        }
    }
}

impl ChipParamSlot for TextParamSlot {
    fn display_text(&self) -> String {
        self.value.clone()
    }

    fn render_editor(&mut self, ui: &mut egui::Ui) -> bool {
        let resp = ui.text_edit_singleline(&mut self.value);
        resp.changed()
    }
}