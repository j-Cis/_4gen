use eframe::egui::{self, Rect, Sense, Vec2};
use crate::theme::hue;

pub fn draw_close_button(ui: &mut egui::Ui) -> (bool, Rect) {
	let (rect, resp) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::click());
	let bg = if resp.hovered() {
		hue::WINDOW_CLOSE_FG
	} else {
		hue::WINDOW_CLOSE_BG
	};
	let stroke_col = if resp.hovered() {
		hue::WINDOW_HEADER_TEXT
	} else {
		hue::WINDOW_CLOSE_FG
	};

	ui.painter().rect_filled(rect, 4.0, bg);
	ui.painter().line_segment(
		[rect.min + Vec2::splat(4.0), rect.max - Vec2::splat(4.0)],
		egui::Stroke::new(1.5, stroke_col),
	);
	ui.painter().line_segment(
		[
			egui::pos2(rect.right() - 4.0, rect.top() + 4.0),
			egui::pos2(rect.left() + 4.0, rect.bottom() - 4.0),
		],
		egui::Stroke::new(1.5, stroke_col),
	);

	(resp.on_hover_text("Zamknij").clicked(), rect)
}