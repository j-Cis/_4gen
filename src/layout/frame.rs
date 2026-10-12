// src/layout/frame.rs

use crate::layout::{
    workspace::Workspace,
    canvas::CanvasMode,
    frame_bar::FrameBar,
    frame_setup::{build_left_bar, build_right_bar},
};
use crate::theme::{hue, ico};
use eframe::egui::{self, Align, Layout, Panel};

pub struct FrameApp<'a> {
    left_bar: FrameBar<'a>,
    right_bar: FrameBar<'a>,
    board: Workspace,
    status: String,
}

impl<'a> FrameApp<'a> {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&_cc.egui_ctx);
        Self {
            left_bar: build_left_bar(),
            right_bar: build_right_bar(),
            board: Workspace::default(),
            status: "Gotowy".into(),
        }
    }
}

impl<'a> eframe::App for FrameApp<'a> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ribbon_frame = egui::Frame::default()
            .fill(hue::MAIN_TOOLBAR_BG)
            .inner_margin(egui::Margin {
                left: ico::ICON_PADDING_CROSS_AXIS as i8,
                right: ico::ICON_PADDING_CROSS_AXIS as i8,
                top: ico::ICON_PADDING_MAIN_AXIS as i8,
                bottom: ico::ICON_PADDING_MAIN_AXIS as i8,
            });

        let ribbon_total_width = ico::ICON_BUTTON_SIZE + (ico::ICON_PADDING_CROSS_AXIS * 2.0);

        Panel::bottom("status_bar")
            .resizable(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label("_4gen · wersja rozwojowa");
                    });
                });
            });

        // LEWY PASEK -> Przełącza wyłącznie tło (WorkspaceCanvas)
        Panel::left("left_ribbon")
            .frame(ribbon_frame)
            .default_size(ribbon_total_width)
            .resizable(false)
            .show(ui, |ui| {
                for id in self.left_bar.show(ui) {
                    match id {
                        "files" => {
                            self.board.set_canvas_mode(CanvasMode::Assets);
                            self.status = "Widok: Zasoby i Pliki".into();
                        }
                        "search" => {
                            self.board.set_canvas_mode(CanvasMode::Individuals);
                            self.status = "Widok: Rejestr Osobowy".into();
                        }
                        _ => {}
                    }
                }
            });

        // PRAWY PASEK -> Będzie sterował oknami narzędziowymi/pływającymi
        Panel::right("right_ribbon")
            .frame(ribbon_frame)
            .default_size(ribbon_total_width)
            .resizable(false)
            .show(ui, |ui| {
                for id in self.right_bar.show(ui) {
                    self.status = match id {
                        "layers" => "Warstwy".into(),
                        "properties" => "Właściwości".into(),
                        "settings" => "Ustawienia".into(),
                        _ => self.status.clone(),
                    };
                }
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(hue::APP_BG))
            .show(ui, |ui| {
                self.board.show(ui);
            });
    }
}