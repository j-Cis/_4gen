use eframe::egui::{self, CursorIcon, Id, Rect, Sense, Vec2};

pub fn handle_header_drag(ui: &mut egui::Ui, rect: Rect, window_id: Id, is_movable: bool) -> Vec2 {
	if is_movable {
		let drag_resp = ui.interact(rect, window_id.with("hdr_drag"), Sense::drag());

		// Ustawienie odpowiedniej ikony kursora w zależności od stanu interakcji
		if drag_resp.dragged() || drag_resp.is_pointer_button_down_on() {
			ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
		} else if drag_resp.hovered() {
			ui.ctx().set_cursor_icon(CursorIcon::Grab);
		}

		drag_resp.drag_delta()
	} else {
		Vec2::ZERO
	}
}