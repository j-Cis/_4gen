use eframe::egui::{self, Image, Vec2};

// Rozmiary ikon i przycisków
pub const ICON_SIZE: f32 = 21.0;
pub const ICON_BUTTON_SIZE: f32 = 34.0;
pub const ICON_BUTTON_CORNER_RADIUS: f32 = 10.0;

// Odstępy (wzorowane na przeglądarce Edge)
pub const ICON_PADDING_CROSS_AXIS: f32 = 4.0; // Odstęp od krawędzi paska po bokach
pub const ICON_PADDING_MAIN_AXIS: f32 = 2.0; // Odstęp między przyciskami w pionie

pub const ICON_SIZE_VEC: Vec2 = Vec2::new(ICON_SIZE, ICON_SIZE);
pub const ICON_BUTTON_SIZE_VEC: Vec2 = Vec2::new(ICON_BUTTON_SIZE, ICON_BUTTON_SIZE);

macro_rules! register_icon {
    ($func_name:ident, $file:literal) => {
        pub fn $func_name<'a>() -> Image<'a> {
            Image::new(egui::include_image!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/ico/",
                $file
            )))
        }
    };
}

// --- CENTRALNY REJESTR IKON ---
// Od teraz w programie wywołujesz po prostu ico::file(), ico::gear() itd.
register_icon!(diamond, "twemoji_1f4a0.svg");
register_icon!(calendar, "twemoji_1f4c6.svg");
register_icon!(ledger, "twemoji_1f4d2.svg");
register_icon!(notebook, "twemoji_1f4d4.svg");
register_icon!(book, "twemoji_1f4d6.svg");
register_icon!(books, "twemoji_1f4da.svg");
register_icon!(scroll, "twemoji_1f4dc.svg");
register_icon!(memo, "twemoji_1f4dd.svg");
register_icon!(magnifier, "twemoji_1f50e.svg");
register_icon!(map, "twemoji_1f5fa.svg");
register_icon!(tools, "twemoji_1f6e0.svg");
register_icon!(puzzle, "twemoji_1f9e9.svg");
register_icon!(compass, "twemoji_1f9ed.svg");
register_icon!(toolbox, "twemoji_1f9f0.svg");
register_icon!(file, "twemoji_1f9fe.svg");
register_icon!(ladder, "twemoji_1fa9b.svg");
register_icon!(brick, "twemoji_1faa2.svg");
register_icon!(hatch, "twemoji_1faaa.svg");
register_icon!(female, "twemoji_2640.svg");
register_icon!(male, "twemoji_2642.svg");
register_icon!(gear, "twemoji_2699.svg");
