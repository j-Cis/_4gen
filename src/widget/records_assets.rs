// src/widget/records_assets.rs

use eframe::egui;

pub struct RecordsAssetsWidget;

impl RecordsAssetsWidget {
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("📦 Rejestr Zasobów (Assets)");
        ui.separator();
        ui.label("Pliki, źródła i dokumenty archiwalne:");
        
        ui.add_space(8.0);
        ui.label("• 📄 Dokumentacja_4gen.xml");
        ui.label("• 📜 Akt_Urodzenia_1892.pdf");
        ui.label("• 🖼️ Archiwum_Zdjec_01.zip");
    }
}