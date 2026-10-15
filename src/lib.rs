// ./src/lib.rs

pub mod app;

#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> eframe::Result {
	eframe::run_native(
		"_4gen",
		eframe::NativeOptions::default(),
		Box::new(|cc| Ok(Box::new(app::App4Gen::new(cc)))),
	)
}

pub mod theme {
	pub mod hue;
	pub mod ico;
	pub mod tab;
}

pub mod pipe {
	pub mod egui_color;
	pub mod icon_button;
}

pub mod widget {
	pub mod records_assets;
	pub mod records_individuals;
}

pub mod layout {
	pub mod setup; 
	pub mod frame;
	pub mod table; 
	pub mod input;
	pub mod system_status;
}