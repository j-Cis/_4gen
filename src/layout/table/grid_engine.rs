// src/layout/table/grid_engine.rs

use eframe::egui::{self, vec2};
use crate::layout::table::grid_render::{header, pinned, scroll};
use crate::layout::table::types::{PinPosition, RowPinPosition, TableConfig, TableState};
use crate::theme::tab::TableTheme;

pub struct TableGridEngine;

impl TableGridEngine {
	pub fn render<F>(
		ui: &mut egui::Ui,
		state: &mut TableState,
		config: &TableConfig,
		theme: &TableTheme,
		row_indices: &[usize],
		mut cell_renderer: F,
	) where
		F: FnMut(usize, &str, &mut egui::Ui),
	{
		ui.style_mut().interaction.selectable_labels = false;
		
		let active_order = state.get_computed_columns_order();

		let mut left_cols = Vec::new();
		let mut center_cols = Vec::new();
		let mut right_cols = Vec::new();

		for id in &active_order {
			if let Some(col) = state.columns.get(id) {
				match col.pinned {
					PinPosition::Left => left_cols.push(id.clone()),
					PinPosition::None => center_cols.push(id.clone()),
					PinPosition::Right => right_cols.push(id.clone()),
				}
			}
		}

		// Rozbicie wierszy na 3 sekcje: góra, środek, dół
		let mut top_rows = Vec::new();
		let mut middle_rows = Vec::new();
		let mut bottom_rows = Vec::new();

		for &idx in row_indices {
			match state.row_pin(idx) {
				RowPinPosition::Top => top_rows.push(idx),
				RowPinPosition::None => middle_rows.push(idx),
				RowPinPosition::Bottom => bottom_rows.push(idx),
			}
		}

		let shift_pressed = ui.input(|i| state.is_shift_active(i));
		let ctrl_pressed = ui.input(|i| state.is_ctrl_active(i));
		let alt_pressed = ui.input(|i| state.is_alt_active(i));

		let left_total_width: f32 = left_cols
			.iter()
			.map(|id| state.columns.get(id).map(|c| c.width_px).unwrap_or(100.0))
			.sum();

		let right_total_width: f32 = right_cols
			.iter()
			.map(|id| state.columns.get(id).map(|c| c.width_px).unwrap_or(100.0))
			.sum();

		let scroll_x_id = ui.scope_id().with("table_horizontal_scroll_sync");
		let mut scroll_x = ui.data(|d| d.get_temp::<f32>(scroll_x_id).unwrap_or(0.0));

		let scroll_y_id = ui.scope_id().with("table_vertical_scroll_sync");
		let mut scroll_y = ui.data(|d| d.get_temp::<f32>(scroll_y_id).unwrap_or(0.0));

		// ==========================================
		// 1. NAGŁÓWKI (ZAMROŻONE W PIONIE)
		// ==========================================
		ui.allocate_ui_with_layout(
			vec2(ui.available_width(), theme.header_height),
			egui::Layout::left_to_right(egui::Align::Center),
			|ui| {
				ui.spacing_mut().item_spacing.x = 0.0;

				if !left_cols.is_empty() {
					ui.allocate_ui_with_layout(
						vec2(left_total_width, theme.header_height),
						egui::Layout::left_to_right(egui::Align::Center),
						|ui| {
							header::render_header_row(ui, &left_cols, state, theme, shift_pressed, ctrl_pressed, alt_pressed);
						},
					);
					ui.separator();
				}

				if !center_cols.is_empty() {
					let center_width = if right_cols.is_empty() {
						ui.available_width()
					} else {
						(ui.available_width() - right_total_width - 1.0).max(0.0)
					};

					let header_scroll = egui::ScrollArea::horizontal()
						.id_salt("header_hscroll")
						.horizontal_scroll_offset(scroll_x)
						.auto_shrink([false, false])
						.max_width(center_width)
						.max_height(theme.header_height)
						.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
						.show(ui, |ui| {
							header::render_header_row(ui, &center_cols, state, theme, shift_pressed, ctrl_pressed, alt_pressed);
						});

					if (header_scroll.state.clamped_offset().x - scroll_x).abs() > 0.01 {
						scroll_x = header_scroll.state.clamped_offset().x;
					}
				}

				if !right_cols.is_empty() {
					ui.separator();
					ui.allocate_ui_with_layout(
						vec2(right_total_width, theme.header_height),
						egui::Layout::left_to_right(egui::Align::Center),
						|ui| {
							header::render_header_row(ui, &right_cols, state, theme, shift_pressed, ctrl_pressed, alt_pressed);
						},
					);
				}
			},
		);

		// ==========================================
		// 2. WIERSZE PRZYPIĘTE DO GÓRY (ZAMROŻONE W PIONIE)
		// ==========================================
		if !top_rows.is_empty() {
			let top_rows_height = top_rows.len() as f32 * theme.row_height;
			ui.allocate_ui_with_layout(
				vec2(ui.available_width(), top_rows_height),
				egui::Layout::left_to_right(egui::Align::Min),
				|ui| {
					ui.spacing_mut().item_spacing.x = 0.0;

					if !left_cols.is_empty() {
						ui.allocate_ui_with_layout(
							vec2(left_total_width, top_rows_height),
							egui::Layout::top_down(egui::Align::LEFT),
							|ui| {
								pinned::render_pinned_body(
									ui, &left_cols, state, config, theme, &top_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
								);
							},
						);
						ui.separator();
					}

					if !center_cols.is_empty() {
						let center_width = if right_cols.is_empty() {
							ui.available_width()
						} else {
							(ui.available_width() - right_total_width - 1.0).max(0.0)
						};

						let top_scroll = egui::ScrollArea::horizontal()
							.id_salt("grid_top_pinned_hscroll")
							.horizontal_scroll_offset(scroll_x)
							.auto_shrink([false, false])
							.max_width(center_width)
							.max_height(top_rows_height)
							.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
							.show(ui, |ui| {
								ui.vertical(|ui| {
									scroll::render_scroll_body(
										ui, &center_cols, state, config, theme, &top_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
									);
								});
							});

						if (top_scroll.state.clamped_offset().x - scroll_x).abs() > 0.01 {
							scroll_x = top_scroll.state.clamped_offset().x;
						}
					}

					if !right_cols.is_empty() {
						ui.separator();
						ui.allocate_ui_with_layout(
							vec2(right_total_width, top_rows_height),
							egui::Layout::top_down(egui::Align::LEFT),
							|ui| {
								pinned::render_pinned_body(
									ui, &right_cols, state, config, theme, &top_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
								);
							},
						);
					}
				},
			);
			ui.separator();
		}

		// ==========================================
		// 3. CIAŁO TABELI (PRZEWIJANE W PIONIE I W POZIOMIE)
		// ==========================================
		let bottom_rows_height = if !bottom_rows.is_empty() {
			bottom_rows.len() as f32 * theme.row_height + 4.0
		} else {
			0.0
		};
		let middle_body_height = (ui.available_height() - bottom_rows_height).max(0.0);

		ui.allocate_ui_with_layout(
			vec2(ui.available_width(), middle_body_height),
			egui::Layout::left_to_right(egui::Align::Min),
			|ui| {
				ui.spacing_mut().item_spacing.x = 0.0;

				if !left_cols.is_empty() {
					ui.allocate_ui_with_layout(
						vec2(left_total_width, middle_body_height),
						egui::Layout::top_down(egui::Align::LEFT),
						|ui| {
							let pinned_scroll = egui::ScrollArea::vertical()
								.id_salt("grid_pinned_vscroll")
								.vertical_scroll_offset(scroll_y)
								.auto_shrink([false, false])
								.max_height(middle_body_height)
								.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
								.show(ui, |ui| {
									pinned::render_pinned_body(
										ui, &left_cols, state, config, theme, &middle_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
									);
								});

							if (pinned_scroll.state.clamped_offset().y - scroll_y).abs() > 0.01 {
								scroll_y = pinned_scroll.state.clamped_offset().y;
							}
						},
					);
					ui.separator();
				}

				if !center_cols.is_empty() {
					let center_width = if right_cols.is_empty() {
						ui.available_width()
					} else {
						(ui.available_width() - right_total_width - 1.0).max(0.0)
					};

					let body_scroll = egui::ScrollArea::both()
						.id_salt("grid_body_scroll")
						.horizontal_scroll_offset(scroll_x)
						.vertical_scroll_offset(scroll_y)
						.auto_shrink([false, false])
						.max_width(center_width)
						.max_height(middle_body_height)
						.show(ui, |ui| {
							ui.vertical(|ui| {
								scroll::render_scroll_body(
									ui, &center_cols, state, config, theme, &middle_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
								);
							});
						});

					if (body_scroll.state.clamped_offset().x - scroll_x).abs() > 0.01 {
						scroll_x = body_scroll.state.clamped_offset().x;
					}
					if (body_scroll.state.clamped_offset().y - scroll_y).abs() > 0.01 {
						scroll_y = body_scroll.state.clamped_offset().y;
					}
				}

				if !right_cols.is_empty() {
					ui.separator();
					ui.allocate_ui_with_layout(
						vec2(right_total_width, middle_body_height),
						egui::Layout::top_down(egui::Align::LEFT),
						|ui| {
							let pinned_scroll_right = egui::ScrollArea::vertical()
								.id_salt("grid_pinned_vscroll_right")
								.vertical_scroll_offset(scroll_y)
								.auto_shrink([false, false])
								.max_height(middle_body_height)
								.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
								.show(ui, |ui| {
									pinned::render_pinned_body(
										ui, &right_cols, state, config, theme, &middle_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
									);
								});

							if (pinned_scroll_right.state.clamped_offset().y - scroll_y).abs() > 0.01 {
								scroll_y = pinned_scroll_right.state.clamped_offset().y;
							}
						},
					);
				}
			},
		);

		// ==========================================
		// 4. WIERSZE PRZYPIĘTE DO DOŁU (ZAMROŻONE W PIONIE)
		// ==========================================
		if !bottom_rows.is_empty() {
			ui.separator();
			let actual_bottom_height = bottom_rows.len() as f32 * theme.row_height;
			ui.allocate_ui_with_layout(
				vec2(ui.available_width(), actual_bottom_height),
				egui::Layout::left_to_right(egui::Align::Min),
				|ui| {
					ui.spacing_mut().item_spacing.x = 0.0;

					if !left_cols.is_empty() {
						ui.allocate_ui_with_layout(
							vec2(left_total_width, actual_bottom_height),
							egui::Layout::top_down(egui::Align::LEFT),
							|ui| {
								pinned::render_pinned_body(
									ui, &left_cols, state, config, theme, &bottom_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
								);
							},
						);
						ui.separator();
					}

					if !center_cols.is_empty() {
						let center_width = if right_cols.is_empty() {
							ui.available_width()
						} else {
							(ui.available_width() - right_total_width - 1.0).max(0.0)
						};

						let bottom_scroll = egui::ScrollArea::horizontal()
							.id_salt("grid_bottom_pinned_hscroll")
							.horizontal_scroll_offset(scroll_x)
							.auto_shrink([false, false])
							.max_width(center_width)
							.max_height(actual_bottom_height)
							.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
							.show(ui, |ui| {
								ui.vertical(|ui| {
									scroll::render_scroll_body(
										ui, &center_cols, state, config, theme, &bottom_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
									);
								});
							});

						if (bottom_scroll.state.clamped_offset().x - scroll_x).abs() > 0.01 {
							scroll_x = bottom_scroll.state.clamped_offset().x;
						}
					}

					if !right_cols.is_empty() {
						ui.separator();
						ui.allocate_ui_with_layout(
							vec2(right_total_width, actual_bottom_height),
							egui::Layout::top_down(egui::Align::LEFT),
							|ui| {
								pinned::render_pinned_body(
									ui, &right_cols, state, config, theme, &bottom_rows, shift_pressed, ctrl_pressed, alt_pressed, &mut cell_renderer,
								);
							},
						);
					}
				},
			);
		}

		ui.data_mut(|d| {
			d.insert_temp(scroll_x_id, scroll_x);
			d.insert_temp(scroll_y_id, scroll_y);
		});
	}
}