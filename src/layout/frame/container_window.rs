use eframe::egui::{self, Pos2, Vec2};
use crate::layout::frame::{
	default_min_window_size, default_window_size, render_container_window,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContainerWindowMode {
	Floating,
	FloatingModal,
	ScreenModal,
}

pub struct ContainerWindow {
	pub id: String,
	pub title: String,
	pub icon: String,
	pub mode: ContainerWindowMode,
	pub is_open: bool,
	pub is_minimized: bool,
	pub is_movable: bool,
	pub is_resizable: bool,
	pub default_pos: Option<Pos2>,
	pub default_size: Option<Vec2>,
	pub min_size: Vec2,
}

impl ContainerWindow {
	pub fn new(
		id: impl Into<String>,
		title: impl Into<String>,
		icon: impl Into<String>,
		mode: ContainerWindowMode,
	) -> Self {
		Self {
			id: id.into(),
			title: title.into(),
			icon: icon.into(),
			mode,
			is_open: false,
			is_minimized: false,
			is_movable: true,
			is_resizable: true,
			default_pos: None,
			default_size: Some(default_window_size()),
			min_size: default_min_window_size(),
		}
	}

	pub fn open(&mut self) {
		self.is_open = true;
	}

	pub fn close(&mut self) {
		self.is_open = false;
	}

	pub fn toggle(&mut self) {
		self.is_open = !self.is_open;
	}

	pub fn with_pos(mut self, pos: Pos2) -> Self {
		self.default_pos = Some(pos);
		self
	}

	pub fn with_size(mut self, size: Vec2) -> Self {
		self.default_size = Some(size);
		self
	}

	pub fn formatted_title(&self) -> String {
		if self.icon.is_empty() {
			self.title.clone()
		} else {
			format!("{} {}", self.icon, self.title)
		}
	}

	pub fn show<F>(&mut self, ui: &mut egui::Ui, add_contents: F)
	where
		F: FnOnce(&mut egui::Ui),
	{
		if !self.is_open {
			return;
		}
		render_container_window(self, ui, add_contents);
	}
}