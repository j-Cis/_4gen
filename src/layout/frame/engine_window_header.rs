use eframe::egui::{self, pos2, Frame, Id, Margin, Rect};
use crate::layout::frame::{
	draw_close_button, draw_minimize_button, handle_header_drag, HeaderAction,
	WindowClosePosition, WindowHeaderConfig, WindowMinimizePosition,
};
use crate::theme::hue;

pub fn draw_custom_header(
	ui: &mut egui::Ui,
	window_id: Id,
	title: &str,
	is_movable: bool,
	header_config: &WindowHeaderConfig,
) -> HeaderAction {
	let mut action = HeaderAction::default();

	let header_frame = Frame::new()
		.fill(hue::WINDOW_HEADER_BG)
		.inner_margin(Margin::symmetric(8, 4));

	let mut left_buttons_max_x = f32::MIN;
	let mut right_buttons_min_x = f32::MAX;

	let header_response = header_frame
		.show(ui, |header_ui| {
			header_ui.horizontal(|ui| {
				ui.spacing_mut().item_spacing.x = 4.0;

				let close_is_left = header_config.close_pos == WindowClosePosition::Left;
				let min_pos = header_config.minimize_pos;

				// 1. Lewa strona nagłówka
				if close_is_left {
					if min_pos == WindowMinimizePosition::BesideLeft {
						let (clicked, rect) = draw_minimize_button(ui);
						if clicked {
							action.toggle_minimize = true;
						}
						left_buttons_max_x = left_buttons_max_x.max(rect.max.x);
					}
					let (clicked, rect) = draw_close_button(ui);
					if clicked {
						action.close_clicked = true;
					}
					left_buttons_max_x = left_buttons_max_x.max(rect.max.x);

					if min_pos == WindowMinimizePosition::BesideRight {
						let (clicked, rect) = draw_minimize_button(ui);
						if clicked {
							action.toggle_minimize = true;
						}
						left_buttons_max_x = left_buttons_max_x.max(rect.max.x);
					}
				} else if min_pos == WindowMinimizePosition::Opposite {
					let (clicked, rect) = draw_minimize_button(ui);
					if clicked {
						action.toggle_minimize = true;
					}
					left_buttons_max_x = left_buttons_max_x.max(rect.max.x);
				}

				// 2. Prawa strona nagłówka i tytuł
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |r_ui| {
					if !close_is_left {
						if min_pos == WindowMinimizePosition::BesideRight {
							let (clicked, rect) = draw_minimize_button(r_ui);
							if clicked {
								action.toggle_minimize = true;
							}
							right_buttons_min_x = right_buttons_min_x.min(rect.min.x);

							let (clicked, rect) = draw_close_button(r_ui);
							if clicked {
								action.close_clicked = true;
							}
							right_buttons_min_x = right_buttons_min_x.min(rect.min.x);
						} else if min_pos == WindowMinimizePosition::BesideLeft {
							let (clicked, rect) = draw_close_button(r_ui);
							if clicked {
								action.close_clicked = true;
							}
							right_buttons_min_x = right_buttons_min_x.min(rect.min.x);

							let (clicked, rect) = draw_minimize_button(r_ui);
							if clicked {
								action.toggle_minimize = true;
							}
							right_buttons_min_x = right_buttons_min_x.min(rect.min.x);
						} else {
							let (clicked, rect) = draw_close_button(r_ui);
							if clicked {
								action.close_clicked = true;
							}
							right_buttons_min_x = right_buttons_min_x.min(rect.min.x);
						}
					} else if min_pos == WindowMinimizePosition::Opposite {
						let (clicked, rect) = draw_minimize_button(r_ui);
						if clicked {
							action.toggle_minimize = true;
						}
						right_buttons_min_x = right_buttons_min_x.min(rect.min.x);
					}

					r_ui.with_layout(
						egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true),
						|t_ui| {
							t_ui.label(
								egui::RichText::new(title)
									.color(hue::WINDOW_HEADER_TEXT)
									.strong(),
							);
						},
					);
				});
			});
		})
		.response;

	let header_rect = header_response.rect;

	// Początek strefy przeciągania za lewymi przyciskami
	let drag_min_x = if left_buttons_max_x > f32::MIN {
		left_buttons_max_x
	} else {
		header_rect.min.x
	};

	// Koniec strefy przeciągania przed prawymi przyciskami
	let drag_max_x = if right_buttons_min_x < f32::MAX {
		right_buttons_min_x
	} else {
		header_rect.max.x
	};

	// Obszar przeciągania obejmuje wyłącznie wolną przestrzeń belki
	if is_movable && drag_max_x > drag_min_x {
		let drag_rect = Rect::from_min_max(
			pos2(drag_min_x, header_rect.min.y),
			pos2(drag_max_x, header_rect.max.y),
		);
		action.drag_delta = handle_header_drag(ui, drag_rect, window_id, true);
	}

	action
}