// src/layout/system_status.rs

use eframe::egui;

#[derive(Clone, Copy, Default, Debug)]
pub struct SystemModifiers {
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
}

pub struct SystemStatus {
	pub status_msg: String,
	pub modifiers: SystemModifiers,
}

impl Default for SystemStatus {
	fn default() -> Self {
		Self {
			status_msg: "Gotowy".into(),
			modifiers: SystemModifiers::default(),
		}
	}
}

impl SystemStatus {
	pub fn show(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(&self.status_msg);

			ui.add_space(16.0);

			let (phys_shift, phys_ctrl, phys_alt) = ui.input(|i| (
				i.modifiers.shift,
				i.modifiers.command || i.modifiers.ctrl,
				i.modifiers.alt,
			));

			let ctrl_active = self.modifiers.ctrl || phys_ctrl;
			if ui.add(egui::Button::new("⌘ Ctrl").selected(ctrl_active)).clicked() {
				self.modifiers.ctrl = !self.modifiers.ctrl;
			}

			let shift_active = self.modifiers.shift || phys_shift;
			if ui.add(egui::Button::new("⇧ Shift").selected(shift_active)).clicked() {
				self.modifiers.shift = !self.modifiers.shift;
			}

			let alt_active = self.modifiers.alt || phys_alt;
			if ui.add(egui::Button::new("⌥ Alt").selected(alt_active)).clicked() {
				self.modifiers.alt = !self.modifiers.alt;
			}

			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
				ui.label("_4gen · wersja rozwojowa · v:0.1.0-dev.2-(2026-42-4::21:27:59:45:04)");
			});
		});
	}
}