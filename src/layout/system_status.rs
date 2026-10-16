// src/layout/system_status.rs

use eframe::egui::{self, Color32, CursorIcon, Id, Order, Pos2, Rect, Sense, Stroke, Vec2};

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
		// 1. Zwykły pasek stanu na dole
		ui.horizontal(|ui| {
			const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

			if self.status_msg == "Gotowy" || self.status_msg.is_empty() {
				ui.label(BUILD_TIMESTAMP);
			} else {
				ui.label(BUILD_TIMESTAMP);
				ui.separator();
				ui.label(&self.status_msg);
			}

			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
				ui.label(concat!("_4gen · wersja rozwojowa · v:", env!("CARGO_PKG_VERSION")));
			});
		});

		// 2. Pływająca siatka 2x2 w lewym dolnym rogu (obok przełącznika paneli)
		self.show_floating_modifiers(ui.ctx());
	}

	fn show_floating_modifiers(&mut self, ctx: &egui::Context) {
		let screen_rect = ctx.input(|i| i.raw.screen_rect.unwrap_or(Rect::EVERYTHING));

		let cell_size = Vec2::new(58.0, 36.0);
		let gap = 5.0;

		// Pozycja X: 8.0 (margines) + 38.0 (szerokość guzika paneli) + 8.0 (odstęp) = 54.0
		let trigger_x = 54.0;
		// Pozycja Y zrównana z guzikiem paneli (screen_rect.max.y - 70.0)
		let trigger_y = (screen_rect.max.y - 69.0).max(screen_rect.min.y);

		let pos_2a = Pos2::new(trigger_x, trigger_y);
		let pos_top_left = Pos2::new(pos_2a.x, pos_2a.y - cell_size.y - gap);

		let (phys_shift, phys_ctrl, phys_alt) = ctx.input(|i| (
			i.modifiers.shift,
			i.modifiers.command || i.modifiers.ctrl,
			i.modifiers.alt,
		));

		let area_pos = if self.menu_expanded { pos_top_left } else { pos_2a };

		egui::Area::new(Id::unique("floating_system_modifiers_grid"))
			.order(Order::Tooltip)
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
						Self::render_trigger_button(ui, ctx, cell_size, true, &mut self.menu_expanded);

						let alt_active = self.modifiers.alt || phys_alt;
						let alt_btn = egui::Button::new("⌥ Alt").min_size(cell_size).selected(alt_active);
						if ui.add(alt_btn).on_hover_text("Wirtualny Alt (2B)").clicked() {
							self.modifiers.alt = !self.modifiers.alt;
						}
					});
				} else {
					// --- STAN ZWINIĘTY: Tylko 2A ---
					Self::render_trigger_button(ui, ctx, cell_size, false, &mut self.menu_expanded);
				}
			});
	}

	fn render_trigger_button(
		ui: &mut egui::Ui,
		ctx: &egui::Context,
		size: Vec2,
		is_open: bool,
		expanded_state: &mut bool,
	) {
		let (rect, resp) = ui.allocate_exact_size(size, Sense::click());

		if resp.clicked() {
			*expanded_state = !*expanded_state;
		}

		if resp.hovered() {
			ctx.set_cursor_icon(CursorIcon::PointingHand);
		}

		// Identyczna paleta kolorystyczna jak floating_nav_toggle
		let bg_color = if resp.is_pointer_button_down_on() {
			Color32::from_rgba_unmultiplied(22, 24, 28, 245)
		} else if resp.hovered() || is_open {
			Color32::from_rgba_unmultiplied(42, 48, 60, 230)
		} else {
			Color32::from_rgba_unmultiplied(32, 34, 38, 175)
		};

		let border_stroke = if resp.hovered() || is_open {
			Stroke::new(1.5, Color32::from_rgba_unmultiplied(255, 255, 255, 180))
		} else {
			Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60))
		};

		let icon_col = if resp.hovered() || is_open {
			Color32::WHITE
		} else {
			Color32::from_rgb(200, 205, 215)
		};

		let corner_radius = 6.0;

		// Cień za przyciskiem
		ui.painter().rect_filled(
			rect.translate(Vec2::new(0.0, 1.5)),
			corner_radius,
			Color32::from_black_alpha(70),
		);
		// Tło i obwódka
		ui.painter().rect_filled(rect, corner_radius, bg_color);
		ui.painter().rect_stroke(rect, corner_radius, border_stroke, egui::StrokeKind::Inside);

		// Sama ikona bez żadnego tekstu
		let icon_text = if is_open { "✕" } else { "⌨" };
		ui.painter().text(
			rect.center(),
			egui::Align2::CENTER_CENTER,
			icon_text,
			egui::FontId::proportional(16.0),
			icon_col,
		);

		resp.on_hover_text(if is_open {
			"Ukryj klawisze modyfikatorów"
		} else {
			"Pokaż klawisze modyfikatorów (Ctrl, Shift, Alt)"
		});
	}
}