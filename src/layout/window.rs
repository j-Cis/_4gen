// src/layout/window.rs

use eframe::egui;

#[derive(PartialEq, Clone, Copy)]
pub enum WindowType {
    Floating,
    Modal,
}

pub struct WorkspaceWindow {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub window_type: WindowType,
    pub is_open: bool,
    pub is_movable: bool,
}

impl WorkspaceWindow {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: impl Into<String>,
        window_type: WindowType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: icon.into(),
            window_type,
            is_open: false,
            is_movable: true,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn show<F>(&mut self, ui: &mut egui::Ui, add_contents: F)
    where
        F: FnOnce(&mut egui::Ui, &mut bool),
    {
        if !self.is_open {
            return;
        }

        let full_title = if self.icon.is_empty() {
            self.title.clone()
        } else {
            format!("{} {}", self.icon, self.title)
        };

        let mut window_open = self.is_open;
        let mut content_open = true;

        if self.window_type == WindowType::Modal {
            // KLUCZOWA ZMIANA: Pobieramy wymiary CAŁEGO ekranu aplikacji, a nie tylko planszy
            let screen_bounds = ui
                .ctx()
                .input(|i| i.raw.screen_rect.unwrap_or(egui::Rect::EVERYTHING));

            // 1. TARCZA OCHRONNA (BACKDROP) NA CAŁĄ APLIKACJĘ
            egui::Area::new(egui::Id::new(&self.id).with("backdrop"))
                .order(egui::Order::Foreground)
                .fixed_pos(screen_bounds.min) // Zaczynamy od rogu ekranu aplikacji
                .show(ui.ctx(), |backdrop_ui| {
                    // Strefa absorbuje WSZYSTKIE kliknięcia z CAŁEGO ekranu (w tym z pasków bocznych)
                    let (rect, _resp) = backdrop_ui
                        .allocate_exact_size(screen_bounds.size(), egui::Sense::click_and_drag());

                    // Rysujemy przyciemnienie na całym ekranie
                    backdrop_ui.painter().rect_filled(
                        rect,
                        0.0,
                        egui::Color32::from_black_alpha(180),
                    );
                });

            // 2. SAMO OKNO MODALNE NA WIERZCHU
            egui::Area::new(egui::Id::new(&self.id))
                .order(egui::Order::Tooltip)
                .pivot(egui::Align2::CENTER_CENTER)
                .fixed_pos(screen_bounds.center()) // Idealnie na środku okna aplikacji
                .show(ui.ctx(), |modal_ui| {
                    egui::Frame::window(modal_ui.style()).show(modal_ui, |content_ui| {
                        content_ui.horizontal(|title_ui| {
                            title_ui.strong(&full_title);
                            title_ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |right_ui| {
                                    if right_ui.button("×").clicked() {
                                        window_open = false;
                                    }
                                },
                            );
                        });
                        content_ui.separator();

                        add_contents(content_ui, &mut content_open);
                    });
                });
        } else {
            // --- ZWYKŁE OKNO PŁYWAJĄCE ---
            let window = egui::Window::new(full_title)
                .id(egui::Id::new(&self.id))
                .constrain_to(ui.max_rect()) // Pływające zamykamy WYŁĄCZNIE na planszy, by nie zasłaniały pasków
                .open(&mut window_open)
                .collapsible(true)
                .resizable(true)
                .movable(self.is_movable);

            window.show(ui.ctx(), |window_ui| {
                add_contents(window_ui, &mut content_open);
            });
        }

        self.is_open = window_open && content_open;
    }
}
