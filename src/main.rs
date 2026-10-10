use eframe::egui;
use x::element::side_menu::{MenuItem, SideMenu};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 500.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Aplikacja _4gen",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}

struct App {
    menu: SideMenu,
    menu_items: Vec<MenuItem>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            menu: SideMenu::new(
                egui::Color32::from_hex("#072b47").unwrap(),
                48.0,
                200.0,
                true,
                Some(0),
            ),
            menu_items: vec![
                MenuItem { icon: "🏠".into(), text: "Główna".into() },
                MenuItem { icon: "⚙".into(), text: "Ustawienia".into() },
                MenuItem { icon: "📊".into(), text: "Statystyki".into() },
            ],
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Główny układ poziomy na całą szerokość i wysokość okna
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                // 1. Menu boczne
                if let Some(clicked_idx) = self.menu.show(ui, &self.menu_items) {
                    println!("Zmieniono zakładkę na: {}", self.menu_items[clicked_idx].text);
                }

                // 2. Główny obszar zawartości
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    ui.heading("Zawartość zakładki");
                    
                    if let Some(idx) = self.menu.active_index {
                        ui.label(format!("Aktywny widok: {}", self.menu_items[idx].text));
                    }
                });
            },
        );
    }
}