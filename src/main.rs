use eframe::egui;

// Importujemy z Twojego modułu: Listwę, Struktury ORAZ nasze własne makro `i!`
use x::{element::side_menu::{MenuItem, SideMenu}, i};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 500.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Aplikacja _4gen",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    )
}

struct App<'a> {
    menu: SideMenu,
    menu_items: Vec<MenuItem<'a>>,
}

impl<'a> Default for App<'a> {
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
                MenuItem { icon: i!("twemoji/1f4d6").into(), text: "Źródła".into() },      // 📖
                MenuItem { icon: i!("twemoji/1f4da").into(), text: "Kolekcje".into() },    // 📚
                MenuItem { icon: i!("twemoji/1faaa").into(), text: "Osoby".into() },       // 🪪
                MenuItem { icon: i!("twemoji/1f4c6").into(), text: "Wydarzenia".into() },  // 🗓️
                MenuItem { icon: i!("twemoji/1f5fa").into(), text: "Lokalizacje".into() }, // 🗺️
               // MenuItem { icon: "🔥".into(),                text: "Zwykły Tekst".into() },// Przykład klasycznego emoji
                MenuItem { icon: i!("twemoji/1f4d2").into(), text: "Notatki".into() },     // 📜
                MenuItem { icon: i!("twemoji/1f9e9").into(), text: "Załączniki".into() },  // 🧩
                MenuItem { icon: i!("twemoji/1fa9b").into(), text: "Konfiguracja".into() },// ⚙️
            ],
        }
    }
}

impl<'a> eframe::App for App<'a> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                if let Some(clicked_idx) = self.menu.show(ui, &self.menu_items) {
                    println!("Zmieniono zakładkę na: {}", self.menu_items[clicked_idx].text);
                }

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