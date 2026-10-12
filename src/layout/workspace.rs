// src/layout/workspace.rs

use eframe::egui;
use crate::layout::{
    canvas::{WorkspaceCanvas, CanvasMode},
    window::{WorkspaceWindow, WindowType},
};

pub struct Workspace {
    pub canvas: WorkspaceCanvas,
    pub welcome_window: WorkspaceWindow,
    pub settings_modal: WorkspaceWindow,
}

impl Default for Workspace {
    fn default() -> Self {
        let mut welcome_window = WorkspaceWindow::new(
            "welcome_win",
            "Witaj w _4gen",
            "👋",
            WindowType::Floating,
        );
        welcome_window.open();

        let settings_modal = WorkspaceWindow::new(
            "settings_modal",
            "Uwaga: Wymagana akcja",
            "⚠️",
            WindowType::Modal,
        );

        Self {
            canvas: WorkspaceCanvas::default(),
            welcome_window,
            settings_modal,
        }
    }
}

impl Workspace {
    pub fn set_canvas_mode(&mut self, mode: CanvasMode) {
        self.canvas.mode = mode;
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // 1. TŁO (WorkspaceCanvas) - Rysujemy zawsze na samym spodu tła Workspace
        self.canvas.show(ui);

        // 2. OKNA PŁYWAJĄCE - Pływają NAD płótnem/tłem
        self.welcome_window.show(ui, |ui, content_open| {
            ui.label("To okno pływa dokładnie NAD wybranym widgetem tła!");
            ui.add_space(10.0);
            if ui.button("Otwórz okno modalne").clicked() {
                self.settings_modal.open();
            }
            if ui.button("Zamknij to okno").clicked() {
                *content_open = false;
            }
        });

        // 3. OKNA MODALNE - Przykrywają całą aplikację
        self.settings_modal.show(ui, |ui, content_open| {
            ui.label("To okno modalne zablokowało całą aplikację.");
            ui.add_space(15.0);
            if ui.button("Rozumiem, zamknij").clicked() {
                *content_open = false;
            }
        });
    }
}