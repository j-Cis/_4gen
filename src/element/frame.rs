use eframe::egui;

#[derive(PartialEq)]
pub enum FrameType {
    Modal,
    Floating,
}

pub struct CustomFrame {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub frame_type: FrameType,
    pub is_open: bool,
}

impl CustomFrame {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: impl Into<String>,
        frame_type: FrameType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: icon.into(),
            frame_type,
            is_open: false,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn show<F>(&mut self, ctx: &egui::Context, add_contents: F)
    where
        F: FnOnce(&mut egui::Ui, &mut bool),
    {
        if !self.is_open {
            return;
        }

        let full_title = format!("{} {}", self.icon, self.title);

        // 1. Zmienna dla krzyżyka w nagłówku Egui (zewnętrzna)
        let mut window_open = self.is_open;
        // 2. Zmienna dla Twojego kodu wewnątrz ramki (wewnętrzna)
        let mut content_open = true;

        let mut window = egui::Window::new(full_title)
            .id(egui::Id::new(&self.id))
            .open(&mut window_open); // Egui "pożycza" window_open

        match self.frame_type {
            FrameType::Modal => {
                window = window
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .movable(false);
            }
            FrameType::Floating => {
                window = window.collapsible(true).resizable(true).movable(true);
            }
        }

        window.show(ctx, |ui| {
            // Wstrzykujemy bezpieczną zmienną do wnętrza ramki
            add_contents(ui, &mut content_open);
        });

        // 3. Ramka pozostaje otwarta tylko jeśli ŻADEN z mechanizmów jej nie zamknął
        self.is_open = window_open && content_open;
    }
}
