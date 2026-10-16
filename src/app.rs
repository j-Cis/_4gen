// src/app.rs

use std::collections::HashMap;
use crate::layout::{
	frame::{
		show_floating_nav_toggle, ContainerCanvas, ContainerWindow, ContainerWindowMode, SideNav,
	},
	setup::{build_nav_side_left, build_nav_side_right, build_view_registry},
	system_status::SystemStatus,
};
use crate::theme::{hue, ico};
use crate::widget::{
	records_assets::RecordsAssetsWidget,
	records_individuals::RecordsIndividualsWidget,
};
use eframe::egui::{self, Align, Layout, Panel, Ui};

// ============================================================================
// REJESTR WIDOKÓW CENTRALNYCH
// ============================================================================
pub trait CanvasView {
	fn title(&self) -> &'static str;
	fn show(&mut self, ui: &mut Ui, sys: &mut SystemStatus);
}

impl CanvasView for RecordsIndividualsWidget {
	fn title(&self) -> &'static str {
		"Rejestr Osobowy"
	}

	fn show(&mut self, ui: &mut Ui, sys: &mut SystemStatus) {
		self.show(ui, sys);
	}
}

impl CanvasView for RecordsAssetsWidget {
	fn title(&self) -> &'static str {
		"Zasoby i Pliki"
	}

	fn show(&mut self, ui: &mut Ui, _sys: &mut SystemStatus) {
		RecordsAssetsWidget::show(ui);
	}
}

pub struct ViewRegistry {
	views: HashMap<&'static str, Box<dyn CanvasView>>,
}

impl ViewRegistry {
	pub fn new() -> Self {
		Self {
			views: HashMap::new(),
		}
	}

	pub fn register(&mut self, id: &'static str, view: Box<dyn CanvasView>) {
		self.views.insert(id, view);
	}

	pub fn title(&self, id: &str) -> Option<&'static str> {
		self.views.get(id).map(|v| v.title())
	}

	pub fn show(&mut self, id: &str, ui: &mut Ui, sys: &mut SystemStatus) {
		if let Some(view) = self.views.get_mut(id) {
			view.show(ui, sys);
		} else {
			ui.centered_and_justified(|ui| {
				ui.label(
					eframe::egui::RichText::new("Wybierz widok z lewego paska")
						.size(18.0)
						.weak(),
				);
			});
		}
	}
}

impl Default for ViewRegistry {
	fn default() -> Self {
		Self::new()
	}
}

// ============================================================================
// GŁÓWNA APLIKACJA
// ============================================================================

pub struct App4Gen<'a> {
	left_bar: SideNav<'a>,
	right_bar: SideNav<'a>,
	views: ViewRegistry,
	active_view_id: String,
	welcome_window: ContainerWindow,
	settings_modal: ContainerWindow,
	system: SystemStatus,
	side_nav_visible: bool,
}

impl<'a> App4Gen<'a> {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		egui_extras::install_image_loaders(&_cc.egui_ctx);

		let mut fonts = egui::FontDefinitions::default();

		fonts.font_data.insert(
			"iosevka_etoile".to_owned(),
			egui::FontData::from_static(include_bytes!("../assets/font/IosevkaEtoile-Regular.ttf")).into(),
		);

		fonts
			.families
			.entry(egui::FontFamily::Proportional)
			.or_default()
			.insert(0, "iosevka_etoile".to_owned());

		fonts
			.families
			.entry(egui::FontFamily::Monospace)
			.or_default()
			.insert(0, "iosevka_etoile".to_owned());

		_cc.egui_ctx.set_fonts(fonts);

		let mut welcome_window = ContainerWindow::new(
			"welcome_win",
			"Witaj w _4gen",
			"👋",
			ContainerWindowMode::Floating,
		);
		welcome_window.open();

		let settings_modal = ContainerWindow::new(
			"settings_modal",
			"Uwaga: Wymagana akcja",
			"⚠️",
			ContainerWindowMode::ScreenModal,
		);

		Self {
			left_bar: build_nav_side_left(),
			right_bar: build_nav_side_right(),
			views: build_view_registry(),
			active_view_id: "search".into(),
			welcome_window,
			settings_modal,
			system: SystemStatus::default(),
			side_nav_visible: true,
		}
	}
}

impl<'a> eframe::App for App4Gen<'a> {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		let ribbon_frame = egui::Frame::default()
			.fill(hue::MAIN_TOOLBAR_BG)
			.inner_margin(egui::Margin {
				left: ico::ICON_PADDING_CROSS_AXIS as i8,
				right: ico::ICON_PADDING_CROSS_AXIS as i8,
				top: ico::ICON_PADDING_MAIN_AXIS as i8,
				bottom: ico::ICON_PADDING_MAIN_AXIS as i8,
			});

		let ribbon_total_width = ico::ICON_BUTTON_SIZE + (ico::ICON_PADDING_CROSS_AXIS * 2.0);

		// PASEK STANU (DÓŁ)
		Panel::bottom("status_bar")
			.resizable(false)
			.show(ui, |ui| {
				self.system.show(ui);
			});

		if self.side_nav_visible {
			Panel::left("left_ribbon")
				.frame(ribbon_frame)
				.default_size(ribbon_total_width)
				.resizable(false)
				.show(ui, |ui| {
					for id in self.left_bar.show(ui) {
						if let Some(title) = self.views.title(id) {
							self.active_view_id = id.to_string();
							self.system.status_msg = format!("Widok: {}", title);
						}
					}
				});

			Panel::right("right_ribbon")
				.frame(ribbon_frame)
				.default_size(ribbon_total_width)
				.resizable(false)
				.show(ui, |ui| {
					for id in self.right_bar.show(ui) {
						self.system.status_msg = match id {
							"layers" => "Warstwy".into(),
							"properties" => "Właściwości".into(),
							"settings" => {
								self.settings_modal.open();
								"Ustawienia".into()
							}
							_ => self.system.status_msg.clone(),
						};
					}
				});
		}

		// OBSZAR CENTRALNY
		egui::CentralPanel::default()
			.frame(egui::Frame::default().fill(hue::APP_BG))
			.show(ui, |ui| {
				ContainerCanvas::show(ui, |ui| {
					self.views.show(&self.active_view_id, ui, &mut self.system);
				});

				let mut open_modal = false;
				self.welcome_window.show(ui, |ui| {
					ui.label("To okno pływa dokładnie NAD wybranym widgetem tła!");
					ui.add_space(10.0);
					if ui.button("Otwórz okno modalne").clicked() {
						open_modal = true;
					}
				});
				if open_modal {
					self.settings_modal.open();
				}

				let mut close_modal = false;
				self.settings_modal.show(ui, |ui| {
					ui.label("To okno modalne zablokowało całą aplikację.");
					ui.add_space(15.0);
					if ui.button("Rozumiem, zamknij").clicked() {
						close_modal = true;
					}
				});
				if close_modal {
					self.settings_modal.close();
				}
			});

		show_floating_nav_toggle(ui.ctx(), &mut self.side_nav_visible);
	}
}