// src/layout/table/config/render.rs

use eframe::egui::{self, Color32, Frame, Id, Stroke, vec2};
use crate::layout::frame::{ContainerWindow, ContainerWindowMode};
use crate::layout::table::config::engine;
use crate::layout::table::types::{ConfigColItem, PinPosition, TableState};
use crate::layout::setup::build_table_config_record;
use crate::theme::tab::TableIcons;

pub struct TableConfigWindow {
	pub window: ContainerWindow,
}

impl Default for TableConfigWindow {
	fn default() -> Self {
		Self {
			window: ContainerWindow::new(
				"table_col_config",
				"Konfiguracja Kolumn",
				TableIcons::CONFIG,
				ContainerWindowMode::Floating,
			),
		}
	}
}

impl TableConfigWindow {
	pub fn show(&mut self, ui: &mut egui::Ui, state: &mut TableState) {
		self.window.show(ui, |ui| {
			ui.label("Zaznacz widoczność, przypięcie (Początek/Koniec) oraz kolejność:");
			ui.add_space(8.0);

			let mut from_idx: Option<usize> = None;
			let mut to_idx: Option<usize> = None;
			let mut swap_action: Option<(usize, usize)> = None;

			let frame = Frame::default().inner_margin(4.0);
			let layout_order = build_table_config_record();

			// Podział konfiguracji na elementy przed Title (lewa strona) i po Title (prawa strona)
			let title_pos = layout_order.iter().position(|x| *x == ConfigColItem::Title).unwrap_or(layout_order.len());
			let left_items = &layout_order[..title_pos];
			let right_items = if title_pos < layout_order.len() { &layout_order[title_pos + 1..] } else { &[] };

			ui.dnd_drop_zone::<usize, ()>(frame, |ui| {
				ui.set_min_size(vec2(350.0, 100.0));

				let display_order = state.display_order.clone();
				let list_len = display_order.len();

				for (row_idx, col_id) in display_order.iter().enumerate() {
					let item_id = Id::unique("col_dnd_item").with(row_idx);
					let mut handle_response: Option<egui::Response> = None;

					ui.horizontal(|ui| {
						ui.spacing_mut().item_spacing.x = 8.0;

						// 1. Elementy lewe (przed Title) w sztywnych kolumnach
						for item in left_items {
							render_item(ui, *item, row_idx, list_len, col_id, state, &mut swap_action);
						}

						// 2. Elementy po Title kotwiczone do prawej krawędzi, a Title wypełnia środek
						if !right_items.is_empty() {
							ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |r_ui| {
								r_ui.spacing_mut().item_spacing.x = 8.0;

								for item in right_items.iter().rev() {
									render_item(r_ui, *item, row_idx, list_len, col_id, state, &mut swap_action);
								}

								r_ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |t_ui| {
									handle_response = Some(render_title_handle(t_ui, item_id, row_idx, col_id, state));
								});
							});
						} else {
							handle_response = Some(render_title_handle(ui, item_id, row_idx, col_id, state));
						}
					});

					// Wizualizacja przeciągania DND na klocku tytułowym
					if let Some(hr) = handle_response {
						if let (Some(pointer), Some(hovered_payload)) = (
							ui.input(|i| i.pointer.interact_pos()),
							hr.dnd_hover_payload::<usize>(),
						) {
							let rect = hr.rect;
							let stroke = Stroke::new(2.0, Color32::from_rgb(0, 120, 215));

							let insert_target = if *hovered_payload == row_idx {
								ui.painter().hline(rect.x_range(), rect.center().y, stroke);
								row_idx
							} else if pointer.y < rect.center().y {
								ui.painter().hline(rect.x_range(), rect.top(), stroke);
								row_idx
							} else {
								ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
								row_idx + 1
							};

							if let Some(dragged_payload) = hr.dnd_release_payload() {
								from_idx = Some(*dragged_payload);
								to_idx = Some(insert_target);
							}
						}
					}

					ui.separator();
				}
			});

			// Akcja przycisków góra/dół (zamiana sąsiadów)
			if let Some((a, b)) = swap_action {
				state.display_order.swap(a, b);
			}

			// Akcja przeciągania myszą DND
			if let (Some(from), Some(to)) = (from_idx, to_idx) {
				engine::reorder_columns(state, from, to);
			}
		});
	}
}

fn render_title_handle(ui: &mut egui::Ui, item_id: Id, row_idx: usize, col_id: &str, state: &TableState) -> egui::Response {
	let available_w = ui.available_width().max(40.0);
	ui.dnd_drag_source(item_id, row_idx, |ui| {
		ui.allocate_ui_with_layout(
			vec2(available_w, 20.0),
			egui::Layout::left_to_right(egui::Align::Center),
			|ui| {
				ui.label("☰");
				if let Some(col) = state.columns.get(col_id) {
					ui.label(&col.title);
				}
			},
		);
	}).response
}

fn render_item(
	ui: &mut egui::Ui,
	item: ConfigColItem,
	row_idx: usize,
	list_len: usize,
	col_id: &str,
	state: &mut TableState,
	swap_action: &mut Option<(usize, usize)>,
) {
	match item {
		ConfigColItem::Visible => {
			ui.allocate_ui_with_layout(
				vec2(22.0, 20.0),
				egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
				|ui| {
					if state.columns.contains_key(col_id) {
						let mut is_visible = state.visible_columns.contains(col_id);
						if ui.checkbox(&mut is_visible, "").changed() {
							engine::toggle_column_visibility(state, col_id, is_visible);
						}
					}
				},
			);
		}
		ConfigColItem::Pinned => {
			// Sztywna kolumna na oba przyciski: dokładnie 72px (dwa sloty po 34px + 4px odstępu)
			ui.allocate_ui_with_layout(
				vec2(72.0, 20.0),
				egui::Layout::left_to_right(egui::Align::Center),
				|ui| {
					ui.spacing_mut().item_spacing.x = 4.0;
					if let Some(col) = state.columns.get(col_id) {
						let is_left = col.pinned == PinPosition::Left;
						let is_right = col.pinned == PinPosition::Right;

						// Przycisk lewego przypięcia 📌L
						let left_tooltip = if is_right {
							"Odepnij z prawej, aby móc przypiąć z lewej"
						} else if is_left {
							"Odepnij z początku (lewo)"
						} else {
							"Przypnij na początku (lewo)"
						};
						if ui
							.selectable_label(is_left, "📌L")
							.on_hover_text(left_tooltip)
							.clicked()
						{
							engine::toggle_pin_left(state, col_id);
						}

						// Przycisk prawego przypięcia 📌R
						let right_tooltip = if is_left {
							"Odepnij z lewej, aby móc przypiąć z prawej"
						} else if is_right {
							"Odepnij z końca (prawo)"
						} else {
							"Przypnij na końcu (prawo)"
						};
						if ui
							.selectable_label(is_right, "📌R")
							.on_hover_text(right_tooltip)
							.clicked()
						{
							engine::toggle_pin_right(state, col_id);
						}
					}
				},
			);
		}
		ConfigColItem::Position => {
			ui.allocate_ui_with_layout(
				vec2(48.0, 20.0),
				egui::Layout::left_to_right(egui::Align::Center),
				|ui| {
					ui.spacing_mut().item_spacing.x = 4.0;

					if row_idx > 0 {
						let btn = egui::Button::new(TableIcons::UP).min_size(vec2(22.0, 18.0));
						if ui.add(btn).on_hover_text("Przesuń w górę").clicked() {
							*swap_action = Some((row_idx, row_idx - 1));
						}
					} else {
						ui.allocate_exact_size(vec2(22.0, 18.0), egui::Sense::hover());
					}

					if row_idx + 1 < list_len {
						let btn = egui::Button::new(TableIcons::DOWN).min_size(vec2(22.0, 18.0));
						if ui.add(btn).on_hover_text("Przesuń w dół").clicked() {
							*swap_action = Some((row_idx, row_idx + 1));
						}
					} else {
						ui.allocate_exact_size(vec2(22.0, 18.0), egui::Sense::hover());
					}
				},
			);
		}
		ConfigColItem::Title => {}
	}
}