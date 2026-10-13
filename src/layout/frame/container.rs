// src/layout/frame/container.rs

use eframe::egui::{
    self, Align, Align2, Color32, Frame, Id, Layout, Pos2, Rect, Sense, Vec2,
};

/// 4 Tryby kontenerów okiennych w aplikacji
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContainerWindowMode {
    /// 1. Pływające niemodalne – interakcja z tłem i paskami jest możliwa.
    ///    Minimalizowalne, zamykalne, suwane za nagłówek.
    Floating,

    /// 2. Pływające modalne – pływa po obszarze centralnym, ale blokuje tło
    ///    i paski nawigacyjne do momentu zamknięcia.
    FloatingModal,

    /// 3. Centralne modalne z mgłą – sztywne (niepływające), idealnie na środku ekranu,
    ///    zaciemnia wszystko ciemną mgłą i blokuje całą resztę aplikacji.
    ScreenModal,
}

// ============================================================================
// A. KONTENER GŁÓWNY (CENTRALNY OBSZAR ROBOCZY MIĘDZY PASKAMI)
// ============================================================================

pub struct ContainerCanvas;

impl ContainerCanvas {
    /// Rysuje uniwersalny obszar centralny. Przyjmuje dowolną zawartość przez domknięcie.
    pub fn show<F>(ui: &mut egui::Ui, add_contents: F)
    where
        F: FnOnce(&mut egui::Ui),
    {
        Frame::default()
            .inner_margin(12.0)
            .show(ui, |ui| {
                add_contents(ui);
            });
    }
}

// ============================================================================
// B. UNIWERSALNY KONTENER OKIENNY (PŁYWAJĄCE I MODALNE)
// ============================================================================

pub struct ContainerWindow {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub mode: ContainerWindowMode,
    pub is_open: bool,
    pub is_minimized: bool,
    pub is_movable: bool,
    pub is_resizable: bool,
    pub default_pos: Option<Pos2>,
    pub default_size: Option<Vec2>,
    pub min_size: Vec2,
}

impl ContainerWindow {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: impl Into<String>,
        mode: ContainerWindowMode,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: icon.into(),
            mode,
            is_open: false,
            is_minimized: false,
            is_movable: true,
            is_resizable: true,
            default_pos: None,
            default_size: Some(Vec2::new(360.0, 240.0)),
            min_size: Vec2::new(200.0, 100.0),
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn with_pos(mut self, pos: Pos2) -> Self {
        self.default_pos = Some(pos);
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.default_size = Some(size);
        self
    }

    fn formatted_title(&self) -> String {
        if self.icon.is_empty() {
            self.title.clone()
        } else {
            format!("{} {}", self.icon, self.title)
        }
    }

    /// Główna metoda renderująca okno w wybranym trybie.
    pub fn show<F>(&mut self, ui: &mut egui::Ui, add_contents: F)
    where
        F: FnOnce(&mut egui::Ui),
    {
        if !self.is_open {
            return;
        }

        let bounds = ui.max_rect();
        let screen_rect = ui
            .ctx()
            .input(|i| i.raw.screen_rect.unwrap_or(egui::Rect::EVERYTHING));

        match self.mode {
            // ----------------------------------------------------------------
            // 1. OKNO PŁYWAJĄCE NIEMODALNE
            // ----------------------------------------------------------------
            ContainerWindowMode::Floating => {
                let mut window_open = self.is_open;
                let mut win = egui::Window::new(self.formatted_title())
                    .id(Id::new(&self.id))
                    .constrain_to(bounds)
                    .open(&mut window_open)
                    .collapsible(true)
                    .resizable(self.is_resizable)
                    .movable(self.is_movable)
                    .min_size(self.min_size);

                if let Some(pos) = self.default_pos {
                    win = win.default_pos(pos);
                }
                if let Some(sz) = self.default_size {
                    win = win.default_size(sz);
                }

                win.show(ui.ctx(), |window_ui| {
                    add_contents(window_ui);
                });

                self.is_open = window_open;
            }

            // ----------------------------------------------------------------
            // 2. OKNO PŁYWAJĄCE MODALNE (BLOKUJE RESZTĘ, ALE SAMO PŁYWA)
            // ----------------------------------------------------------------
            ContainerWindowMode::FloatingModal => {
                // Tarcza blokująca kliknięcia w tle (kontener główny + oba paski boczne)
                egui::Area::new(Id::new(&self.id).with("floating_modal_shield"))
                    .order(egui::Order::Foreground)
                    .fixed_pos(screen_rect.min)
                    .show(ui.ctx(), |shield_ui| {
                        shield_ui.allocate_exact_size(
                            screen_rect.size(),
                            Sense::click_and_drag(),
                        );
                        // Subtelna półprzezroczystość sygnalizująca blokadę
                        shield_ui.painter().rect_filled(
                            screen_rect,
                            0.0,
                            Color32::from_black_alpha(60),
                        );
                    });

                // Odczyt lub inicjalizacja pozycji okna
                let pos_id = Id::new(&self.id).with("window_pos");
                let mut current_pos = ui.data(|d| d.get_temp::<Pos2>(pos_id)).unwrap_or_else(|| {
                    self.default_pos
                        .unwrap_or(bounds.center() - Vec2::new(180.0, 120.0))
                });

                let min_id = Id::new(&self.id).with("is_minimized");
                let mut is_minimized = ui
                    .data(|d| d.get_temp::<bool>(min_id))
                    .unwrap_or(self.is_minimized);

                let mut keep_open = self.is_open;

                // Okno wyrenderowane na warstwie Tooltip ponad tarczą ochronną
                egui::Area::new(Id::new(&self.id).with("floating_modal_body"))
                    .order(egui::Order::Tooltip)
                    .fixed_pos(current_pos)
                    .constrain_to(bounds)
                    .show(ui.ctx(), |modal_ui| {
                        Frame::window(modal_ui.style()).show(modal_ui, |content_ui| {
                            // Belka tytułowa
                            let header_response = content_ui.horizontal(|title_ui| {
                                title_ui.strong(self.formatted_title());
                                title_ui.with_layout(
                                    Layout::right_to_left(Align::Center),
                                    |right_ui| {
                                        if right_ui.button("×").on_hover_text("Zamknij").clicked() {
                                            keep_open = false;
                                        }
                                        let min_sym = if is_minimized { "➕" } else { "➖" };
                                        if right_ui.button(min_sym).on_hover_text("Zwiń / Rozwiń").clicked() {
                                            is_minimized = !is_minimized;
                                        }
                                    },
                                );
                            }).response;

                            // Suwanie TYLKO za pasek nazwy
                            if self.is_movable {
                                let drag_resp = content_ui.interact(
                                    header_response.rect,
                                    Id::new(&self.id).with("header_drag"),
                                    Sense::drag(),
                                );
                                current_pos += drag_resp.drag_delta();
                            }

                            // Treść pokazywana tylko jeśli okno nie jest zminimalizowane
                            if !is_minimized {
                                content_ui.separator();
                                add_contents(content_ui);
                            }
                        });
                    });

                self.is_open = keep_open;
                self.is_minimized = is_minimized;

                ui.data_mut(|d| {
                    d.insert_temp(pos_id, current_pos);
                    d.insert_temp(min_id, is_minimized);
                });
            }

            // ----------------------------------------------------------------
            // 3. OKNO CENTRALNE MODALNE Z MGŁĄ (SZTYWNE, BLOKUJE WSZYSTKO)
            // ----------------------------------------------------------------
            ContainerWindowMode::ScreenModal => {
                // Tarcza zaciemniająca z pełną mgłą
                egui::Area::new(Id::new(&self.id).with("screen_backdrop"))
                    .order(egui::Order::Foreground)
                    .fixed_pos(screen_rect.min)
                    .show(ui.ctx(), |backdrop_ui| {
                        backdrop_ui.allocate_exact_size(
                            screen_rect.size(),
                            Sense::click_and_drag(),
                        );
                        backdrop_ui.painter().rect_filled(
                            screen_rect,
                            0.0,
                            Color32::from_black_alpha(180), // Wyraźna ciemna mgła
                        );
                    });

                let mut keep_open = self.is_open;

                // Okno centralne na warstwie Tooltip
                egui::Area::new(Id::new(&self.id).with("screen_modal_body"))
                    .order(egui::Order::Tooltip)
                    .pivot(Align2::CENTER_CENTER)
                    .fixed_pos(screen_rect.center())
                    .show(ui.ctx(), |modal_ui| {
                        Frame::window(modal_ui.style()).show(modal_ui, |content_ui| {
                            content_ui.horizontal(|title_ui| {
                                title_ui.strong(self.formatted_title());
                                title_ui.with_layout(
                                    Layout::right_to_left(Align::Center),
                                    |right_ui| {
                                        if right_ui.button("×").on_hover_text("Zamknij").clicked() {
                                            keep_open = false;
                                        }
                                    },
                                );
                            });
                            content_ui.separator();
                            add_contents(content_ui);
                        });
                    });

                self.is_open = keep_open;
            }
        }
    }
}