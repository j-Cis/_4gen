// src/layout/system_status.rs

use eframe::egui::{self, Id, Order, Pos2, Rect, Vec2};

#[derive(Clone, Copy, Default, Debug)]
pub struct SystemModifiers {
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
}

pub struct SystemStatus {
	pub status_msg: String,
	pub modifiers: SystemModifiers,
	pub menu_expanded: bool,
}

impl Default for SystemStatus {
	fn default() -> Self {
		Self {
			status_msg: "Gotowy".into(),
			modifiers: SystemModifiers::default(),
			menu_expanded: false,
		}
	}
}

impl SystemStatus {
	pub fn show(&mut self, ui: &mut egui::Ui) {
		// 1. CZYSTY PASEK STANU NA DOLE (Bez ciasnych guzików modyfikatorów)
		ui.horizontal(|ui| {
			ui.label(&self.status_msg);

			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
				ui.label("_4gen · wersja rozwojowa · v:0.1.0-dev.2");
			});
		});

		// 2. NIEPRZYKRYTA SIATKA 2x2 W LEWYM DOLNYM ROGU
		self.show_floating_modifiers(ui.ctx());
	}

	fn show_floating_modifiers(&mut self, ctx: &egui::Context) {
		let screen_rect = ctx.input(|i| i.raw.screen_rect.unwrap_or(Rect::EVERYTHING));

		let cell_size = Vec2::new(58.0, 32.0);
		let gap = 5.0;
		let margin_x = 8.0;
		let margin_bottom = 28.0; // Wysokość tuż nad dolnym paskiem stanu

		// 2A: Pozycja bazowa wyzwalacza (Lewy dolny róg)
		let pos_2a = Pos2::new(
			screen_rect.min.x + margin_x,
			screen_rect.max.y - margin_bottom - cell_size.y,
		);

		// 1A: Pozycja lewego górnego rogu siatki (dla stanu rozwiniętego)
		let pos_top_left = Pos2::new(pos_2a.x, pos_2a.y - cell_size.y - gap);

		let (phys_shift, phys_ctrl, phys_alt) = ctx.input(|i| (
			i.modifiers.shift,
			i.modifiers.command || i.modifiers.ctrl,
			i.modifiers.alt,
		));

		let area_pos = if self.menu_expanded { pos_top_left } else { pos_2a };

		egui::Area::new(Id::unique("floating_system_modifiers_grid"))
			.order(Order::Tooltip) // Zawsze na samej górze - nic go nie przykryje
			.fixed_pos(area_pos)
			.show(ctx, |ui| {
				ui.style_mut().interaction.selectable_labels = false;
				ui.spacing_mut().item_spacing = Vec2::new(gap, gap);

				if self.menu_expanded {
					// --- RZĄD 1: 1A (Ctrl) i 1B (Shift) ---
					ui.horizontal(|ui| {
						let ctrl_active = self.modifiers.ctrl || phys_ctrl;
						let ctrl_btn = egui::Button::new("⌘ Ctrl").min_size(cell_size).selected(ctrl_active);
						if ui.add(ctrl_btn).on_hover_text("Wirtualny Ctrl (1A)").clicked() {
							self.modifiers.ctrl = !self.modifiers.ctrl;
						}

						let shift_active = self.modifiers.shift || phys_shift;
						let shift_btn = egui::Button::new("⇧ Shift").min_size(cell_size).selected(shift_active);
						if ui.add(shift_btn).on_hover_text("Wirtualny Shift (1B)").clicked() {
							self.modifiers.shift = !self.modifiers.shift;
						}
					});

					// --- RZĄD 2: 2A (Wyzwalacz) i 2B (Alt) ---
					ui.horizontal(|ui| {
						let trig_btn = egui::Button::new("✕ SK").min_size(cell_size).selected(true);
						if ui.add(trig_btn).on_hover_text("Zwiń przyciski modyfikatorów (2A)").clicked() {
							self.menu_expanded = false;
						}

						let alt_active = self.modifiers.alt || phys_alt;
						let alt_btn = egui::Button::new("⌥ Alt").min_size(cell_size).selected(alt_active);
						if ui.add(alt_btn).on_hover_text("Wirtualny Alt (2B)").clicked() {
							self.modifiers.alt = !self.modifiers.alt;
						}
					});
				} else {
					// --- STAN ZWINIĘTY: Tylko 2A w lewym dolnym rogu ---
					let trig_btn = egui::Button::new("⌨ SK").min_size(cell_size).selected(false);
					if ui.add(trig_btn).on_hover_text("Rozwiń skróty klawiszowe (Ctrl, Shift, Alt)").clicked() {
						self.menu_expanded = true;
					}
				}
			});
	}
}