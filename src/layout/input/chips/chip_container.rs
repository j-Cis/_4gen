// src/layout/input/chips/chip_container.rs

use eframe::egui::{self, Color32, CursorIcon, Frame, Id, Margin, Order, Stroke, Vec2};
use crate::layout::input::chips::{
	render_autocomplete_input, render_chip, ChipAction, ChipContainerMode, ChipItem, ChipLimits,
	ChipOverflow, ChipSlot,
};

pub struct ChipsContainer<'a> {
	pub id: Id,
	pub items: &'a mut Vec<ChipItem>,
	pub input_buffer: &'a mut String,
	pub autocomplete_pool: &'a [String],
	pub mode: ChipContainerMode,
	pub overflow: ChipOverflow,
	pub limits: ChipLimits,
	pub selected_chip: &'a mut Option<String>,
}

impl<'a> ChipsContainer<'a> {
	pub fn new(
		id: impl Into<Id>,
		items: &'a mut Vec<ChipItem>,
		input_buffer: &'a mut String,
		autocomplete_pool: &'a [String],
		selected_chip: &'a mut Option<String>,
	) -> Self {
		Self {
			id: id.into(),
			items,
			input_buffer,
			autocomplete_pool,
			mode: ChipContainerMode::SingleLine,
			overflow: ChipOverflow::Wrap,
			limits: ChipLimits::default(),
			selected_chip,
		}
	}

	pub fn mode(mut self, mode: ChipContainerMode) -> Self {
		self.mode = mode;
		self
	}

	pub fn overflow(mut self, overflow: ChipOverflow) -> Self {
		self.overflow = overflow;
		self
	}

	pub fn limits(mut self, limits: ChipLimits) -> Self {
		self.limits = limits;
		self
	}

	pub fn show(mut self, ui: &mut egui::Ui) -> Vec<ChipAction> {
		let mut actions = Vec::new();

		let container_frame = Frame::new()
			.fill(Color32::from_rgb(28, 30, 34))
			.stroke(Stroke::new(1.0, Color32::from_gray(60)))
			.corner_radius(6.0)
			.inner_margin(Margin::same(4));

		container_frame.show(ui, |ui| {
			ui.style_mut().interaction.selectable_labels = false;
			ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);

			match (self.mode, self.overflow) {
				(ChipContainerMode::MultiLine, ChipOverflow::Wrap) => {
					ui.horizontal_wrapped(|ui| {
						self.render_chips_flow(ui, 0, self.items.len(), &mut actions);
						self.render_input_slot(ui, &mut actions);
					});
				}

				(ChipContainerMode::SingleLine, ChipOverflow::Scroll) => {
					egui::ScrollArea::horizontal()
						.id_salt(self.id.with("chips_hscroll"))
						.auto_shrink([false, true])
						.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
						.show(ui, |ui| {
							ui.horizontal(|ui| {
								self.render_chips_flow(ui, 0, self.items.len(), &mut actions);
								self.render_input_slot(ui, &mut actions);
							});
						});
				}

				(ChipContainerMode::MultiLine, ChipOverflow::Scroll) => {
					egui::ScrollArea::vertical()
						.id_salt(self.id.with("chips_vscroll"))
						.auto_shrink([false, true])
						.max_height(80.0)
						.show(ui, |ui| {
							ui.horizontal_wrapped(|ui| {
								self.render_chips_flow(ui, 0, self.items.len(), &mut actions);
								self.render_input_slot(ui, &mut actions);
							});
						});
				}

				(_, ChipOverflow::Popup { max_visible }) => {
					let total = self.items.len();
					let visible_count = max_visible.min(total);
					let hidden_count = total.saturating_sub(visible_count);

					ui.horizontal(|ui| {
						self.render_chips_flow(ui, 0, visible_count, &mut actions);

						if hidden_count > 0 {
							let popup_id = self.id.with("hidden_chips_popup");
							let is_open_id = self.id.with("hidden_chips_is_open");
							let mut is_open = ui.data(|d| d.get_temp::<bool>(is_open_id)).unwrap_or(false);

							let badge_text = format!("+{} ▾", hidden_count);
							let btn_resp = ui.selectable_label(is_open, egui::RichText::new(badge_text).strong());
							if btn_resp.hovered() {
								ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
							}

							if btn_resp.clicked() {
								is_open = !is_open;
								ui.data_mut(|d| d.insert_temp(is_open_id, is_open));
							}

							if is_open {
								let popup_pos = btn_resp.rect.left_bottom() + Vec2::new(0.0, 4.0);
								let popup_resp = egui::Area::new(popup_id)
									.fixed_pos(popup_pos)
									.order(Order::Tooltip)
									.show(ui.ctx(), |pop_ui| {
										Frame::popup(pop_ui.style())
											.fill(Color32::from_rgb(32, 34, 38))
											.inner_margin(Margin::same(6))
											.show(pop_ui, |ui| {
												ui.vertical(|ui| {
													self.render_chips_flow(ui, visible_count, total, &mut actions);
												});
											});
									});

								if ui.input(|i| i.pointer.any_pressed())
									&& let Some(pos) = ui.input(|i| i.pointer.interact_pos())
									&& !popup_resp.response.rect.contains(pos)
									&& !btn_resp.rect.contains(pos)
								{
									ui.data_mut(|d| d.insert_temp(is_open_id, false));
								}
							}
						}

						self.render_input_slot(ui, &mut actions);
					});
				}

				_ => {
					ui.horizontal(|ui| {
						self.render_chips_flow(ui, 0, self.items.len(), &mut actions);
						self.render_input_slot(ui, &mut actions);
					});
				}
			}
		});

		for action in &actions {
			if let ChipAction::Removed(id) = action {
				self.items.retain(|c| c.id() != id);
			}
		}

		actions
	}

	fn render_chips_flow(
		&mut self,
		ui: &mut egui::Ui,
		start_idx: usize,
		end_idx: usize,
		actions: &mut Vec<ChipAction>,
	) {
		for idx in start_idx..end_idx {
			if idx >= self.items.len() {
				break;
			}
			let item = &mut self.items[idx];
			let is_selected = self.selected_chip.as_deref() == Some(item.id());
			let out = render_chip(ui, item, is_selected);

			if out.remove_clicked {
				actions.push(ChipAction::Removed(item.id().to_string()));
			} else if out.add_param_clicked {
				item.convert_to_parametric();
				actions.push(ChipAction::Clicked(item.id().to_string()));
			} else if out.param_changed {
				if let Some(val) = item.param_value() {
					actions.push(ChipAction::ParamChanged {
						id: item.id().to_string(),
						display_value: val,
					});
				}
			} else if out.clicked {
				*self.selected_chip = Some(item.id().to_string());
				actions.push(ChipAction::Clicked(item.id().to_string()));
			}
		}
	}

	fn render_input_slot(&mut self, ui: &mut egui::Ui, actions: &mut Vec<ChipAction>) {
		let res = render_autocomplete_input(
			ui,
			self.id.with("autocomplete"),
			self.input_buffer,
			self.autocomplete_pool,
			self.limits.allow_custom,
		);

		if let Some(text) = res.committed_text {
			let id = format!("chip_{}", ui.input(|i| i.time));

			let new_chip = if let Some((key, val)) = text.split_once(':') {
				let k = key.trim();
				let v = val.trim();

				let slot = match k {
					"Płeć" => ChipSlot::select(
						if v.is_empty() { "Kobieta" } else { v },
						vec!["Kobieta".into(), "Mężczyzna".into()],
					),
					"Wiek" => ChipSlot::range(18.0, 65.0, 0.0, 120.0),
					_ => ChipSlot::text(v),
				};

				ChipItem::Parametric {
					id,
					key: k.to_string(),
					slot,
					is_editing: v.is_empty(),
				}
			} else {
				ChipItem::Simple {
					id,
					label: text.clone(),
				}
			};

			if self.limits.can_add(new_chip.key_or_label(), self.items) {
				actions.push(ChipAction::Added(new_chip.id().to_string()));
				self.items.push(new_chip);
			}
		}
	}
}