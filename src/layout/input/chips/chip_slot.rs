// src/layout/input/chips/chip_slot.rs

use eframe::egui::{
	self, vec2, widgets::RangeSlider, Color32, CursorIcon, Key, Sense,
};

// ============================================================================
// 1. SLOT TEKSTOWY (rozszerza się inline w poziomie)
// ============================================================================
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextParamSlot {
	pub value: String,
}

impl TextParamSlot {
	pub fn new(val: impl Into<String>) -> Self {
		Self { value: val.into() }
	}

	pub fn render_inline(&mut self, ui: &mut egui::Ui, is_editing: &mut bool) -> bool {
		let mut changed = false;

		if *is_editing {
			let resp = ui.add(
				egui::TextEdit::singleline(&mut self.value)
					.desired_width(85.0)
					.hint_text("Wpisz..."),
			);
			resp.request_focus();

			let enter_pressed = ui.input(|i| i.key_pressed(Key::Enter));
			let esc_pressed = ui.input(|i| i.key_pressed(Key::Escape));

			let ok_btn = ui.add(
				egui::Button::new(
					egui::RichText::new("✓")
						.color(Color32::from_rgb(100, 255, 100))
						.size(12.0),
				)
				.frame(false),
			);

			if resp.changed() {
				changed = true;
			}

			if ok_btn.clicked() || enter_pressed || (resp.lost_focus() && !ok_btn.hovered()) {
				*is_editing = false;
			} else if esc_pressed {
				*is_editing = false;
			}
		} else {
			let display = if self.value.trim().is_empty() {
				"<?>"
			} else {
				self.value.as_str()
			};

			let resp = ui
				.add(
					egui::Label::new(
						egui::RichText::new(display)
							.color(if self.value.trim().is_empty() {
								Color32::from_rgb(255, 180, 0)
							} else {
								Color32::WHITE
							})
							.size(13.0),
					)
					.selectable(false)
					.sense(Sense::click()),
				)
				.on_hover_text("Kliknij, aby edytować tekst");

			if resp.clicked() {
				*is_editing = true;
			}
			if resp.hovered() {
				ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
			}
		}

		changed
	}
}

// ============================================================================
// 2. SLOT WYBORU / MENU (ComboBox wbudowany w pigułkę)
// ============================================================================
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectParamSlot {
	pub selected: String,
	pub options: Vec<String>,
}

impl SelectParamSlot {
	pub fn new(selected: impl Into<String>, options: Vec<String>) -> Self {
		Self {
			selected: selected.into(),
			options,
		}
	}

	pub fn render_inline(&mut self, ui: &mut egui::Ui, id_salt: &str) -> bool {
		let mut changed = false;

		ui.spacing_mut().button_padding = vec2(4.0, 1.0);
		let combo = egui::ComboBox::from_id_salt(id_salt)
			.selected_text(&self.selected)
			.height(160.0);

		combo.show_ui(ui, |ui| {
			ui.style_mut().interaction.selectable_labels = false;
			for opt in &self.options {
				let resp = ui.selectable_value(&mut self.selected, opt.clone(), opt);
				if resp.clicked() {
					changed = true;
				}
				if resp.hovered() {
					ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
				}
			}
		});

		changed
	}
}

// ============================================================================
// 3. SLOT ZAKRESU (RangeSlider wbudowany w pigułkę)
// ============================================================================
#[derive(Clone, Debug, PartialEq)]
pub struct RangeSliderParamSlot {
	pub low: f32,
	pub high: f32,
	pub min: f32,
	pub max: f32,
}

impl RangeSliderParamSlot {
	pub fn new(low: f32, high: f32, min: f32, max: f32) -> Self {
		Self { low, high, min, max }
	}

	pub fn render_inline(&mut self, ui: &mut egui::Ui, is_editing: &mut bool) -> bool {
		let mut changed = false;

		if *is_editing {
			let slider = RangeSlider::new(&mut self.low, &mut self.high, self.min..=self.max);
			let resp = ui.add(slider);
			if resp.changed() {
				changed = true;
			}

			let ok_btn = ui.add(
				egui::Button::new(
					egui::RichText::new("✓")
						.color(Color32::from_rgb(100, 255, 100))
						.size(12.0),
				)
				.frame(false),
			);

			if ok_btn.clicked() || ui.input(|i| i.key_pressed(Key::Enter)) {
				*is_editing = false;
			}
		} else {
			let label_text = format!("{:.0} - {:.0}", self.low, self.high);
			let resp = ui
				.add(
					egui::Label::new(
						egui::RichText::new(label_text).color(Color32::WHITE).size(13.0),
					)
					.selectable(false)
					.sense(Sense::click()),
				)
				.on_hover_text("Kliknij, aby zmienić zakres");

			if resp.clicked() {
				*is_editing = true;
			}
			if resp.hovered() {
				ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
			}
		}

		changed
	}
}

// ============================================================================
// GŁÓWNY ENUM SLOTU
// ============================================================================
#[derive(Clone, Debug)]
pub enum ChipSlot {
	Text(TextParamSlot),
	Select(SelectParamSlot),
	Range(RangeSliderParamSlot),
}

impl ChipSlot {
	pub fn text(val: impl Into<String>) -> Self {
		Self::Text(TextParamSlot::new(val))
	}

	pub fn select(selected: impl Into<String>, options: Vec<String>) -> Self {
		Self::Select(SelectParamSlot::new(selected, options))
	}

	pub fn range(low: f32, high: f32, min: f32, max: f32) -> Self {
		Self::Range(RangeSliderParamSlot::new(low, high, min, max))
	}

	pub fn display_text(&self) -> String {
		match self {
			Self::Text(s) => s.value.clone(),
			Self::Select(s) => s.selected.clone(),
			Self::Range(s) => format!("{:.0} - {:.0}", s.low, s.high),
		}
	}

	pub fn render_inline(&mut self, ui: &mut egui::Ui, id_salt: &str, is_editing: &mut bool) -> bool {
		match self {
			Self::Text(s) => s.render_inline(ui, is_editing),
			Self::Select(s) => s.render_inline(ui, id_salt),
			Self::Range(s) => s.render_inline(ui, is_editing),
		}
	}
}