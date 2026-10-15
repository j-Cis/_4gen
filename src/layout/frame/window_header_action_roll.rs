use eframe::egui::{self, Rect, Sense, Vec2};
use crate::theme::hue;

pub fn draw_minimize_button(ui: &mut egui::Ui) -> (bool, Rect) {
	let (rect, resp) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::click());
	let bg = if resp.hovered() {
		hue::WINDOW_MINIMIZE_FG
	} else {
		hue::WINDOW_MINIMIZE_BG
	};
	let stroke_col = if resp.hovered() {
		hue::WINDOW_HEADER_TEXT
	} else {
		hue::WINDOW_MINIMIZE_FG
	};

	ui.painter().rect_filled(rect, 4.0, bg);
	let y = rect.center().y;
	ui.painter().line_segment(
		[egui::pos2(rect.left() + 4.0, y), egui::pos2(rect.right() - 4.0, y)],
		egui::Stroke::new(1.5, stroke_col),
	);

	(resp.on_hover_text("Zwiń/Rozwiń").clicked(), rect)
}