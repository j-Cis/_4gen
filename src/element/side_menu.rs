use eframe::egui;

#[derive(Clone)]
pub struct MenuItem {
    pub icon: String,
    pub text: String,
}

pub struct SideMenu {
    pub bg_color: egui::Color32,
    pub icon_size: f32, // Oznacza też bazową szerokość zwiniętego paska
    pub expanded_width: f32,
    pub is_collapsed: bool,
    pub active_index: Option<usize>,
}

impl SideMenu {
    pub fn new(
        bg_color: egui::Color32,
        icon_size: f32,
        expanded_width: f32,
        is_collapsed: bool,
        active_index: Option<usize>,
    ) -> Self {
        Self {
            bg_color,
            icon_size,
            expanded_width,
            is_collapsed,
            active_index,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, items: &[MenuItem]) -> Option<usize> {
        let mut clicked_index = None;
        
        let current_width = if self.is_collapsed {
            self.icon_size
        } else {
            self.expanded_width
        };

        // 1. REZERWACJA MIEJSCA: Zawsze 100% wysokości okna i dokładna szerokość
        let (sidebar_rect, _) = ui.allocate_exact_size(
            egui::vec2(current_width, ui.available_height()), 
            egui::Sense::hover()
        );

        // 2. TŁO PASKA: Rysujemy sami na całym zarezerwowanym prostokącie
        ui.painter().rect_filled(sidebar_rect, 0.0, self.bg_color);

        let mut current_y = sidebar_rect.min.y;

        // 3. PRZYCISK HAMBURGERA (Zwijanie / Rozwijanie)
        let toggle_rect = egui::Rect::from_min_max(
            egui::pos2(sidebar_rect.min.x, current_y),
            egui::pos2(sidebar_rect.max.x, current_y + self.icon_size),
        );
        current_y += self.icon_size;

        let toggle_resp = ui.interact(toggle_rect, ui.id().with("toggle"), egui::Sense::click());
        if toggle_resp.clicked() {
            self.is_collapsed = !self.is_collapsed;
        }
        if toggle_resp.hovered() {
            ui.painter().rect_filled(toggle_rect, 0.0, egui::Color32::from_white_alpha(20));
        }

        // Rysowanie ikony hamburgera - ZAWSZE wyśrodkowana względem icon_size
        let icon_x_center = sidebar_rect.min.x + (self.icon_size / 2.0);
        ui.painter().text(
            egui::pos2(icon_x_center, toggle_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "☰",
            egui::FontId::proportional(22.0),
            egui::Color32::WHITE,
        );

        // Odstęp pod hamburgerem
        current_y += 10.0;

        // 4. ELEMENTY MENU
        for (i, item) in items.iter().enumerate() {
            let is_active = self.active_index == Some(i);
            
            // Rezerwujemy prostokąt na dany przycisk
            let item_rect = egui::Rect::from_min_max(
                egui::pos2(sidebar_rect.min.x, current_y),
                egui::pos2(sidebar_rect.max.x, current_y + self.icon_size),
            );
            current_y += self.icon_size;

            let item_resp = ui.interact(item_rect, ui.id().with(i), egui::Sense::click());
            if item_resp.clicked() {
                self.active_index = Some(i);
                clicked_index = Some(i);
            }

            // Tło po najechaniu lub dla aktywnego elementu (styl Edge)
            if is_active {
                ui.painter().rect_filled(item_rect, 0.0, egui::Color32::from_white_alpha(30));
                
                // Niebieski pionowy pasek po lewej stronie aktywnego elementu
                let accent_rect = egui::Rect::from_min_max(
                    egui::pos2(item_rect.min.x, item_rect.min.y + 10.0),
                    egui::pos2(item_rect.min.x + 3.0, item_rect.max.y - 10.0),
                );
                ui.painter().rect_filled(accent_rect, 2.0, egui::Color32::from_rgb(0, 120, 215));
            } else if item_resp.hovered() {
                ui.painter().rect_filled(item_rect, 0.0, egui::Color32::from_white_alpha(15));
            }

            // Rysowanie ikony - używamy TEJ SAMEJ osi X co hamburger. Ani drgnie.
            let color = if is_active || item_resp.hovered() { 
                egui::Color32::WHITE 
            } else { 
                egui::Color32::from_gray(200) 
            };
            
            ui.painter().text(
                egui::pos2(icon_x_center, item_rect.center().y),
                egui::Align2::CENTER_CENTER,
                &item.icon,
                egui::FontId::proportional(20.0),
                color,
            );

            // Rysowanie tekstu TYLKO gdy menu jest rozwinięte
            if !self.is_collapsed {
                let text_x_start = sidebar_rect.min.x + self.icon_size + 4.0;
                ui.painter().text(
                    egui::pos2(text_x_start, item_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &item.text,
                    egui::FontId::proportional(16.0),
                    color,
                );
            }
        }

        clicked_index
    }
}