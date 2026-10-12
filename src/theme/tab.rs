// src/theme/tab.rs

use eframe::egui;

pub struct TableIcons;

impl TableIcons {
    pub const UP: &'static str = "⤴️";
    pub const DOWN: &'static str = "⤵️";
    pub const CONFIG: &'static str = "📊";
    pub const PREV: &'static str = "◀️";
    pub const NEXT: &'static str = "▶️";
    pub const ADD: &'static str = "➕";
    pub const EDIT: &'static str = "📝";
    pub const DELETE: &'static str = "➖";
    pub const MERGE: &'static str = "🔀";
    pub const DUPLICATE: &'static str = "🔂";
    pub const TAG: &'static str = "🏷️";
}

#[derive(Clone, Copy, Debug)]
pub struct TableTheme {
    pub header_bg: egui::Color32,
    pub row_even_bg: egui::Color32,
    pub row_odd_bg: egui::Color32,
    pub row_hover_bg: egui::Color32,
    pub row_selected_bg: egui::Color32,
    pub toolbar_bg: egui::Color32,
    pub header_height: f32,
    pub row_height: f32,
    pub toolbar_icon_size: f32,
}

impl Default for TableTheme {
    fn default() -> Self {
        Self {
            header_bg: egui::Color32::from_hex("#000000").unwrap(),
            row_even_bg: egui::Color32::from_hex("#202020").unwrap(),
            row_odd_bg: egui::Color32::from_hex("#1a1a1a").unwrap(),
            row_hover_bg: egui::Color32::from_hex("#03121e").unwrap(),
            row_selected_bg: egui::Color32::from_hex("#062741").unwrap(),
            toolbar_bg: egui::Color32::from_hex("#181818").unwrap(),
            header_height: 28.0,
            row_height: 35.0,
            toolbar_icon_size: 18.0,
        }
    }
}