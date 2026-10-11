use eframe::egui;

// --- UKRYTY MECHANIZM ŁADOWANIA IKON ---

/// Makro ułatwiające ładowanie ikon SVG w czasie kompilacji
#[macro_export]
macro_rules! i {
    ($path:literal) => {
        eframe::egui::Image::new(eframe::egui::include_image!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/icons/",
            $path,
            ".svg"
        )))
    };
}

#[derive(Clone)]
pub enum MenuIcon<'a> {
    Text(String),
    Image(egui::Image<'a>),
}

// Pozwala użyć "🪪".into()
impl<'a> From<&'a str> for MenuIcon<'a> {
    fn from(s: &'a str) -> Self {
        MenuIcon::Text(s.to_owned())
    }
}

// Pozwala użyć i!("twemoji/1faaa").into()
impl<'a> From<egui::Image<'a>> for MenuIcon<'a> {
    fn from(img: egui::Image<'a>) -> Self {
        MenuIcon::Image(img)
    }
}

#[derive(Clone)]
pub struct MenuItem<'a> {
    pub icon: MenuIcon<'a>,
    pub text: String,
}

// --- WŁAŚCIWY KOMPONENT MENU ---

pub struct SideMenu {
    pub bg_color: egui::Color32,
    pub icon_size: f32,
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

    pub fn show(&mut self, ui: &mut egui::Ui, items: &[MenuItem<'_>]) -> Option<usize> {
        let mut clicked_index = None;

        let current_width = if self.is_collapsed {
            self.icon_size
        } else {
            self.expanded_width
        };

        let (sidebar_rect, _) = ui.allocate_exact_size(
            egui::vec2(current_width, ui.available_height()),
            egui::Sense::hover(),
        );

        ui.painter().rect_filled(sidebar_rect, 0.0, self.bg_color);

        let mut current_y = sidebar_rect.min.y;

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
            ui.painter()
                .rect_filled(toggle_rect, 0.0, egui::Color32::from_white_alpha(20));
        }

        let icon_x_center = sidebar_rect.min.x + (self.icon_size / 2.0);
        ui.painter().text(
            egui::pos2(icon_x_center, toggle_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "☰",
            egui::FontId::proportional(22.0),
            egui::Color32::WHITE,
        );

        current_y += 10.0;

        for (i, item) in items.iter().enumerate() {
            let is_active = self.active_index == Some(i);

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

            if is_active {
                ui.painter()
                    .rect_filled(item_rect, 0.0, egui::Color32::from_white_alpha(30));

                let accent_rect = egui::Rect::from_min_max(
                    egui::pos2(item_rect.min.x, item_rect.min.y + 10.0),
                    egui::pos2(item_rect.min.x + 3.0, item_rect.max.y - 10.0),
                );
                ui.painter()
                    .rect_filled(accent_rect, 2.0, egui::Color32::from_rgb(0, 120, 215));
            } else if item_resp.hovered() {
                ui.painter()
                    .rect_filled(item_rect, 0.0, egui::Color32::from_white_alpha(15));
            }

            let color = if is_active || item_resp.hovered() {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_gray(200)
            };

            // Rysowanie zależy od tego, jaki typ ikony przekazano
            match &item.icon {
                MenuIcon::Text(text) => {
                    ui.painter().text(
                        egui::pos2(icon_x_center, item_rect.center().y),
                        egui::Align2::CENTER_CENTER,
                        text,
                        egui::FontId::proportional(20.0),
                        color,
                    );
                }
                MenuIcon::Image(img) => {
                    let img_size = self.icon_size * 0.5;
                    let img_rect = egui::Rect::from_center_size(
                        egui::pos2(icon_x_center, item_rect.center().y),
                        egui::vec2(img_size, img_size),
                    );

                    let tint = if is_active || item_resp.hovered() {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::from_white_alpha(150)
                    };

                    img.clone().tint(tint).paint_at(ui, img_rect);
                }
            }

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
