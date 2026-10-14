// src\theme\hue.rs

use crate::pipe::egui_color::oklch;
use egui::Color32;

pub const MAIN_TOOLBAR_BG: Color32 = oklch(0.281, 0.065, 247.0);
pub const MAIN_TOOLBAR_TOOLTIP: Color32 = oklch(0.175, 0.034, 247.0);
pub const MAIN_TOOLBAR_ACTIVE: Color32 = oklch(0.406, 0.102, 247.0);
pub const MAIN_TOOLBAR_HOVER: Color32 = oklch(0.405, 0.163, 247.0);

pub const APP_BG: Color32 = Color32::from_rgb(25, 26, 27);

pub const SUB_TOOLBAR_BG: Color32 = Color32::from_rgb(51, 51, 51);
pub const SUB_TOOLBAR_TOOLTIP: Color32 = Color32::from_rgb(36, 36, 36);
pub const SUB_TOOLBAR_ACTIVE: Color32 = Color32::from_rgb(76, 76, 76);
pub const SUB_TOOLBAR_HOVER: Color32 = Color32::from_rgb(76, 76, 76);

// --- KOLORY KONTENERÓW OKIENNYCH ---

// Przycisk zamykania
pub const WINDOW_CLOSE_BG: Color32 = Color32::from_rgba_unmultiplied_const(237, 28, 36, 45);
pub const WINDOW_CLOSE_FG: Color32 = Color32::from_rgba_unmultiplied_const(237, 28, 36, 115);

// Przycisk zwijania
pub const WINDOW_MINIMIZE_BG: Color32 = Color32::from_rgba_unmultiplied_const(255, 242, 0, 45);
pub const WINDOW_MINIMIZE_FG: Color32 = Color32::from_rgba_unmultiplied_const(255, 242, 0, 115);

// Baza okna
pub const WINDOW_HEADER_BG: Color32 = Color32::from_rgb(32, 33, 35); // Domyślne ciemne tło nagłówka
pub const WINDOW_HEADER_TEXT: Color32 = Color32::from_rgb(220, 220, 220); // Domyślny jasny tekst nagłówka
pub const WINDOW_CONTENT_BG: Color32 = Color32::from_rgb(25, 26, 27); // Tło zawartości (zgodne z APP_BG)